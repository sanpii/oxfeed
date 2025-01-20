pub(crate) mod auth;
pub(crate) mod favicon;
pub(crate) mod icon;
pub(crate) mod item;
pub(crate) mod opml;
pub(crate) mod search;
pub(crate) mod source;
pub(crate) mod tag;
pub(crate) mod user;
pub(crate) mod webhook;
pub(crate) mod websocket;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("").service(index).service(counts)
}

#[actix_web::get("/")]
async fn index(
    elephantry: actix_web::web::Data<elephantry::Pool>,
) -> oxfeed::Result<actix_web::HttpResponse> {
    elephantry.ping()?;

    Ok(actix_web::HttpResponse::NoContent().finish())
}

#[actix_web::get("/counts")]
async fn counts(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;

    let sql = include_str!("../../sql/counts.sql");
    let counts = elephantry.query_one::<oxfeed::Counts>(sql, &[&token])?;
    let response = actix_web::HttpResponse::Ok().json(counts);

    Ok(response)
}
