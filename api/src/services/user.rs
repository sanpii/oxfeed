use oxfeed_common::new_user::{Entity, Model};

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/users").service(create)
}

#[actix_web::post("")]
async fn create(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    data: actix_web::web::Json<Entity>,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    elephantry.insert_one::<Model>(&data.into_inner())?;
    let response = actix_web::HttpResponse::NoContent().finish();

    Ok(response)
}
