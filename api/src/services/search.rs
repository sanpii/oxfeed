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
    sql.push_str("order by ts_rank_cd(f.document, to_tsquery($2))");
    sql.push_str(&query.pagination.to_sql());

    let items = elephantry.query::<oxfeed_common::item::Item>(&sql, &[&token, &query.q])?;

    let mut sql = include_str!("../../sql/search_items_count.sql").to_string();
    sql.push_str(&format!("and {}\n", clause.to_string()));
    let count = elephantry.query_one::<i64>(&sql, &[&token, &query.q])?;

    let pager = elephantry::Pager::new(
        items,
        count as usize,
        query.pagination.page,
        query.pagination.limit,
    );

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

    let mut sql = include_str!("../../sql/search_tags.sql").to_string();
    sql.push_str(&query.pagination.to_sql());

    let q = format!("^{}", query.q);
    let tags = elephantry.query::<String>(&sql, &[&token, &q])?;

    let sql = include_str!("../../sql/search_tags_count.sql");
    let count = elephantry.query_one::<i64>(&sql, &[&token])?;

    let pager = elephantry::Pager::new(
        tags,
        count as usize,
        query.pagination.page,
        query.pagination.limit,
    );

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
