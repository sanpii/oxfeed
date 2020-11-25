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
async fn all(elephantry: Data<elephantry::Pool>, query: Query<Request>) -> crate::Result {
    let params = [&format!(".*{}.*", query.q) as &dyn elephantry::ToSql];
    let clause = elephantry::Where::from("item.title ~* $*", &params);

    super::item::fetch(&elephantry, &clause, &query.pagination)
}

#[actix_web::get("/favorites")]
async fn favorites(elephantry: Data<elephantry::Pool>, query: Query<Request>) -> crate::Result {
    let params = [&format!(".*{}.*", query.q) as &dyn elephantry::ToSql];
    let clause = elephantry::Where::from("item.title ~* $* and favorite", &params);

    super::item::fetch(&elephantry, &clause, &query.pagination)
}

#[actix_web::get("/unread")]
async fn unread(elephantry: Data<elephantry::Pool>, query: Query<Request>) -> crate::Result {
    let params = [&format!(".*{}.*", query.q) as &dyn elephantry::ToSql];
    let clause = elephantry::Where::from("item.title ~* $* and not read", &params);

    super::item::fetch(&elephantry, &clause, &query.pagination)
}

#[actix_web::get("/tags")]
async fn tags(elephantry: Data<elephantry::Pool>, query: Query<Request>) -> crate::Result {
    let sql = r#"
with tags as (
    select unnest(tags) as tag from source
)
select distinct tag
    from tags
    where tag ~* $*
    order by 1;
"#;

    let q = format!("{}.*", query.q);
    let tags = elephantry.query::<String>(sql, &[&q])?;

    let response = actix_web::HttpResponse::Ok().json(tags);

    Ok(response)
}

#[actix_web::get("/sources")]
async fn sources(elephantry: Data<elephantry::Pool>, query: Query<Request>) -> crate::Result {
    let params = [&format!(".*{}.*", query.q) as &dyn elephantry::ToSql];
    let clause = elephantry::Where::from("source.title ~* $*", &params);
    super::source::fetch(&elephantry, &clause, &query.pagination)
}
