use oxfeed::filter::Model;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/filters")
        .service(all)
        .service(create)
        .service(delete)
        .service(update)
}

#[actix_web::get("")]
async fn all(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let model = elephantry.model::<Model>();
    let items = model.all(&token)?;
    let response = actix_web::HttpResponse::Ok().json(items);

    Ok(response)
}

#[actix_web::post("")]
async fn create(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    mut data: actix_web::web::Json<crate::form::Filter>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    use std::convert::TryInto;

    let token = identity.token(&elephantry)?;

    let user = elephantry
        .model::<oxfeed::user::Model>()
        .find_from_token(&token)
        .ok_or(oxfeed::Error::Auth)?;

    data.user_id = Some(user.id);
    let filter = elephantry.insert_one::<Model>(&data.into_inner().try_into()?)?;
    let response = actix_web::HttpResponse::Ok().json(filter);

    Ok(response)
}

#[actix_web::delete("/{filter_id}")]
async fn delete(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    path: actix_web::web::Path<uuid::Uuid>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let filter_id = path.into_inner();
    let token = identity.token(&elephantry)?;
    let response = match elephantry.model::<Model>().delete(&token, &filter_id)? {
        Some(filter) => actix_web::HttpResponse::Ok().json(filter),
        None => actix_web::HttpResponse::NoContent().finish(),
    };

    Ok(response)
}

#[actix_web::put("/{source_id}")]
async fn update(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    mut data: actix_web::web::Json<crate::form::Filter>,
    path: actix_web::web::Path<uuid::Uuid>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    use std::convert::TryInto;

    let token = identity.token(&elephantry)?;

    let user = elephantry
        .model::<oxfeed::user::Model>()
        .find_from_token(&token)
        .ok_or(oxfeed::Error::Auth)?;

    data.user_id = Some(user.id);
    let filter_id = Some(path.into_inner());
    let pk = elephantry::pk!(filter_id);
    let filter = elephantry
        .update_one::<Model>(&pk, &data.into_inner().try_into()?)?
        .ok_or(oxfeed::Error::NotFound)?;

    Ok(actix_web::HttpResponse::Ok().json(filter))
}
