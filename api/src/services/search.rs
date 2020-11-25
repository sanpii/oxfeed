use actix_web::web::{Data, Query};

#[derive(serde::Deserialize)]
struct Request {
    q: String,
}

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/search")
        .service(tags)
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
