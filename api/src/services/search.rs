use actix_web::web::{Data, Query};

#[derive(serde::Deserialize)]
struct Request {
    q: String,
    #[serde(flatten)]
    pagination: super::Pagination,
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
) -> crate::Result {
    let q = format!(".*{}.*", query.q);
    let clause = elephantry::Where::from("item.title ~* $*", vec![&q]);

    super::item::fetch(&elephantry, &identity, &clause, &query.pagination)
}

#[actix_web::get("/favorites")]
async fn favorites(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> crate::Result {
    let q = format!(".*{}.*", query.q);
    let clause = elephantry::Where::from("item.title ~* $* and favorite", vec![&q]);

    super::item::fetch(&elephantry, &identity, &clause, &query.pagination)
}

#[actix_web::get("/unread")]
async fn unread(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> crate::Result {
    let q = format!(".*{}.*", query.q);
    let clause = elephantry::Where::from("item.title ~* $* and not read", vec![&q]);

    super::item::fetch(&elephantry, &identity, &clause, &query.pagination)
}

#[actix_web::get("/tags")]
async fn tags(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> crate::Result {
    let token = match identity.token() {
        Some(token) => token,
        None => return Ok(actix_web::HttpResponse::Unauthorized().finish()),
    };

    let mut sql = include_str!("../sql/search_tags.sql").to_string();
    sql.push_str(&query.pagination.to_sql());

    let q = format!("{}.*", query.q);
    let tags = elephantry.query::<String>(&sql, &[&token, &q])?;

    let sql = include_str!("../sql/search_tags_count.sql");
    let count = elephantry.query_one::<i64>(&sql, &[&token])?;

    let pager = elephantry::Pager::new(
        tags,
        count as usize,
        query.pagination.page(),
        query.pagination.limit(),
    );

    let response = actix_web::HttpResponse::Ok().json(pager);

    Ok(response)
}

#[actix_web::get("/sources")]
async fn sources(
    elephantry: Data<elephantry::Pool>,
    query: Query<Request>,
    identity: crate::Identity,
) -> crate::Result {
    let q = format!(".*{}.*", query.q);
    let clause = elephantry::Where::from("source.title ~* $*", vec![&q]);
    super::source::fetch(&elephantry, &identity, &clause, &query.pagination)
}
