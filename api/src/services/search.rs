use actix_web::web::{Data, Query};

#[derive(serde::Deserialize)]
struct Request {
    active: Option<bool>,
    q: Option<String>,
    tag: Option<String>,
    source: Option<String>,
    #[serde(flatten)]
    pagination: elephantry_extras::Pagination,
}

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/search")
        .service(all)
        .service(favorites)
        .service(unread)
        .service(tags)
        .service(sources)
}

#[actix_web::get("/all")]
async fn all(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    search(&elephantry, &identity, &elephantry::Where::new(), &query)
}

#[actix_web::get("/favorites")]
async fn favorites(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let clause = elephantry::Where::from("favorite", Vec::new());
    search(&elephantry, &identity, &clause, &query)
}

#[actix_web::get("/unread")]
async fn unread(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let clause = elephantry::Where::from("not read", Vec::new());
    search(&elephantry, &identity, &clause, &query)
}

fn search(
    elephantry: &elephantry::Pool,
    identity: &crate::Identity,
    clause: &elephantry::Where,
    query: &Request,
) -> oxfeed::Result<actix_web::HttpResponse> {
    use std::fmt::Write as _;

    let mut clause = clause.clone();

    let mut sql = if let Some(q) = &query.q {
        clause.and_where("document @@ websearch_to_tsquery($*)", vec![q]);
        include_str!("../../sql/fts_items.sql").to_string()
    } else {
        include_str!("../../sql/search_items.sql").to_string()
    };

    if let Some(active) = &query.active {
        clause.and_where("s.active = $*", vec![active]);
    }

    if let Some(tag) = &query.tag {
        clause.and_where("$* = any(tags)", vec![tag]);
    }

    if let Some(source) = &query.source {
        clause.and_where("(s.title ~* $* or s.url ~* $*)", vec![source, source]);
    }

    let token = identity.token(elephantry)?;
    clause.and_where("token = $*", vec![&token]);

    writeln!(sql, "where {clause}").ok();
    writeln!(sql, "group by i.item_id, s.title, s.tags, s.icon").ok();

    if query.q.is_some() {
        writeln!(sql, ", f.document\norder by ts_rank_cd(f.document, websearch_to_tsquery($1)) desc, i.published desc").ok();
    } else {
        writeln!(sql, "order by i.published desc").ok();
    }

    let pager = count::<oxfeed::item::Item>(elephantry, &sql, &clause.params(), &query.pagination)?;
    let response = actix_web::HttpResponse::Ok().json(pager);

    Ok(response)
}

#[actix_web::get("/tags")]
async fn tags(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let sql = include_str!("../../sql/search_tags.sql");
    let q = query.q.as_ref().map(|x| format!("^{x}"));

    let pager = count::<String>(&elephantry, sql, &[&token, &q], &query.pagination)?;
    let response = actix_web::HttpResponse::Ok().json(pager);

    Ok(response)
}

#[actix_web::get("/sources")]
async fn sources(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let q = query.q.clone().unwrap_or_else(|| ".*".to_string());

    let mut clause = elephantry::Where::builder()
        .and_where("source.title ~* $*", vec![&q])
        .build();

    if let Some(active) = &query.active {
        clause.and_where("source.active = $*", vec![active]);
    }

    if let Some(source) = &query.source {
        clause.and_where("source.url ~* $*", vec![source])
    }

    if let Some(tag) = &query.tag {
        clause.and_where("$* = any(source.tags)", vec![tag]);
    }

    super::source::fetch(&elephantry, &identity, &clause, &query.pagination)
}

fn count<T: elephantry::Entity>(
    elephantry: &elephantry::Pool,
    sql: &str,
    params: &[&dyn elephantry::ToSql],
    pagination: &elephantry_extras::Pagination,
) -> oxfeed::Result<elephantry::Pager<T>> {
    let sql_count = format!("with items as ({sql}) select count(items) from items");
    let count = elephantry.query_one::<i64>(&sql_count, params)?;

    let mut sql = sql.to_string();
    sql.push_str(&pagination.to_sql());

    let items = elephantry.query::<T>(&sql, params)?;

    let pager = elephantry::Pager::new(items, count as usize, pagination.page, pagination.limit);

    Ok(pager)
}
