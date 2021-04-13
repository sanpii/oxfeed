use actix_web::web::{Data, Query};

#[derive(serde::Deserialize)]
struct Request {
    q: Option<String>,
    tag: Option<String>,
    #[serde(flatten)]
    pagination: oxfeed_common::Pagination,
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
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    search(&elephantry, &identity, &elephantry::Where::new(), &query)
}

#[actix_web::get("/favorites")]
async fn favorites(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let clause = elephantry::Where::from("favorite", Vec::new());
    search(&elephantry, &identity, &clause, &query)
}

#[actix_web::get("/unread")]
async fn unread(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let clause = elephantry::Where::from("not read", Vec::new());
    search(&elephantry, &identity, &clause, &query)
}

fn search(
    elephantry: &elephantry::Pool,
    identity: &crate::Identity,
    clause: &elephantry::Where,
    query: &Request,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let mut clause = clause.clone();

    let mut sql = if let Some(q) = &query.q {
        clause.and_where("document @@ websearch_to_tsquery($*)", vec![q]);
        include_str!("../../sql/fts_items.sql").to_string()
    } else {
        include_str!("../../sql/search_items.sql").to_string()
    };

    if let Some(tag) = &query.tag {
        clause.and_where("$* = any(tags)", vec![tag]);
    }

    let token = identity.token(&elephantry)?;
    clause.and_where("token = $*", vec![&token]);

    sql.push_str(&format!("where {}\n", clause.to_string()));

    if query.q.is_some() {
        sql.push_str("order by ts_rank_cd(f.document, websearch_to_tsquery($1))\n");
    }

    let pager = count::<oxfeed_common::item::Item>(&elephantry, &sql, &clause.params(), &query.pagination)?;
    let response = actix_web::HttpResponse::Ok().json(pager);

    Ok(response)
}

#[actix_web::get("/tags")]
async fn tags(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let sql = include_str!("../../sql/search_tags.sql");
    let q = query.q.as_ref().map(|x| format!("^{}", x));

    let pager = count::<String>(&elephantry, sql, &[&token, &q], &query.pagination)?;
    let response = actix_web::HttpResponse::Ok().json(pager);

    Ok(response)
}

#[actix_web::get("/sources")]
async fn sources(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let q = query.q.clone().unwrap_or_else(|| ".*".to_string());

    let mut clause = elephantry::Where::builder()
        .and_where("source.title ~* $*", vec![&q])
        .or_where("source.url ~* $*", vec![&q])
        .build();

    if let Some(tag) = &query.tag {
        clause.and_where("$* = any(source.tags)", vec![tag]);
    }

    super::source::fetch(&elephantry, &identity, &clause, &query.pagination)
}

fn count<T: elephantry::Entity>(
    elephantry: &elephantry::Pool,
    sql: &str,
    params: &[&dyn elephantry::ToSql],
    pagination: &oxfeed_common::Pagination,
) -> oxfeed_common::Result<elephantry::Pager<T>> {
    let sql_count = format!("with items as ({}) select count(items) from items", sql);
    let count = elephantry.query_one::<i64>(&sql_count, params)?;

    let mut sql = sql.to_string();
    sql.push_str(&pagination.to_sql());

    let items = elephantry.query::<T>(&sql, params)?;

    let pager = elephantry::Pager::new(
        items,
        count as usize,
        pagination.page,
        pagination.limit,
    );

    Ok(pager)
}
