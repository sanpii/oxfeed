use actix_web::web::{Data, Query};

#[derive(serde::Deserialize)]
struct Request {
    q: String,
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
    let token = identity.token();

    let mut sql = include_str!("../../sql/search_items.sql").to_string();
    sql.push_str(&format!("and {}\n", clause.to_string()));
    sql.push_str("order by ts_rank_cd(f.document, websearch_to_tsquery($2))");

    let pager = count::<oxfeed_common::item::Item>(&elephantry, &sql, &[&token, &query.q], &query.pagination)?;
    let response = actix_web::HttpResponse::Ok().json(pager);

    Ok(response)
}

#[actix_web::get("/tags")]
async fn tags(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token();
    let sql = include_str!("../../sql/search_tags.sql");
    let q = format!("^{}", query.q);

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
    let clause = elephantry::Where::builder()
        .and_where("source.title ~* $*", vec![&query.q])
        .or_where("source.url ~* $*", vec![&query.q])
        .build();
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
