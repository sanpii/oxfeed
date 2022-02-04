use oxfeed_common::user::Model;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/account")
        .service(create)
        .service(delete)
        .service(update)
}

#[actix_web::post("")]
async fn create(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    data: actix_web::web::Json<oxfeed_common::account::Entity>,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    elephantry.insert_one::<oxfeed_common::account::Model>(&data.into_inner())?;
    let response = actix_web::HttpResponse::NoContent().finish();

    Ok(response)
}

#[actix_web::put("")]
async fn update(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    actix_web::web::Json(data): actix_web::web::Json<oxfeed_common::account::Entity>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let user = match elephantry.model::<Model>().find_from_token(&token) {
        Some(user) => user,
        None => return Err(oxfeed_common::Error::Auth),
    };
    elephantry.update_one::<oxfeed_common::account::Model>(
        &elephantry::pk! {user_id => user.id},
        &data,
    )?;
    let response = actix_web::HttpResponse::NoContent().finish();

    Ok(response)
}

#[actix_web::delete("")]
async fn delete(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let user = match elephantry.model::<Model>().find_from_token(&token) {
        Some(user) => user,
        None => return Err(oxfeed_common::Error::Auth),
    };
    elephantry.delete_by_pk::<Model>(&elephantry::pk! {user_id => user.id})?;
    let response = actix_web::HttpResponse::NoContent().finish();

    Ok(response)
}
