pub(crate) mod auth;
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
    actix_web::web::scope("/").service(counts)
}

#[actix_web::get("/counts")]
async fn counts(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;

    let sql = include_str!("../../sql/counts.sql");
    let counts = elephantry.query_one::<oxfeed_common::Counts>(sql, &[&token])?;
    let response = actix_web::HttpResponse::Ok().json(counts);

    Ok(response)
}
