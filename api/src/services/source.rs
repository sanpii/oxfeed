use actix_web::web::{Data, Json, Path};
use oxfeed_common::item::Model as ItemModel;
use oxfeed_common::source::Model;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/sources")
        .service(get)
        .service(delete)
        .service(update)
        .service(all)
        .service(create)
}

#[actix_web::get("")]
async fn all(
    elephantry: Data<elephantry::Pool>,
    pagination: actix_web::web::Query<oxfeed_common::Pagination>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    fetch(
        &elephantry,
        &identity,
        &elephantry::Where::new(),
        &pagination,
    )
}

pub(crate) fn fetch(
    elephantry: &elephantry::Pool,
    identity: &crate::Identity,
    filter: &elephantry::Where,
    pagination: &oxfeed_common::Pagination,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let model = elephantry.model::<Model>();
    let token = identity.token(&elephantry)?;
    let sources = model.all(&token, filter, pagination)?;
    let response = actix_web::HttpResponse::Ok().json(sources);

    Ok(response)
}

#[actix_web::post("")]
async fn create(
    elephantry: Data<elephantry::Pool>,
    mut data: Json<crate::form::Source>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    use std::convert::TryInto;

    let token = identity.token(&elephantry)?;

    let user = match elephantry
        .model::<oxfeed_common::user::Model>()
        .find_from_token(&token)
    {
        Some(user) => user,
        None => return Ok(actix_web::HttpResponse::Unauthorized().finish()),
    };

    data.user_id = Some(user.user_id);
    let source = elephantry.insert_one::<Model>(&data.into_inner().try_into()?)?;
    let response = actix_web::HttpResponse::Ok().json(source);

    Ok(response)
}

#[actix_web::get("/{source_id}")]
async fn get(
    elephantry: Data<elephantry::Pool>,
    item_id: Path<uuid::Uuid>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let response = match elephantry.model::<Model>().one(&item_id, &token)? {
        Some(source) => actix_web::HttpResponse::Ok().json(source),
        None => actix_web::HttpResponse::NotFound().finish(),
    };

    Ok(response)
}

#[actix_web::delete("/{source_id}")]
async fn delete(
    elephantry: Data<elephantry::Pool>,
    path: Path<uuid::Uuid>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let source_id = path.into_inner();

    let token = identity.token(&elephantry)?;

    let source = match elephantry.model::<Model>().one(&token, &source_id)? {
        Some(source) => source,
        None => return Ok(actix_web::HttpResponse::NoContent().finish()),
    };

    elephantry.delete_where::<ItemModel>("source_id = $*", &[&source_id])?;

    let response = match elephantry.delete_one::<Model>(&source)? {
        Some(source) => actix_web::HttpResponse::Ok().json(source),
        None => actix_web::HttpResponse::NoContent().finish(),
    };

    Ok(response)
}

#[actix_web::put("/{source_id}")]
async fn update(
    elephantry: Data<elephantry::Pool>,
    mut data: Json<crate::form::Source>,
    path: Path<uuid::Uuid>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    use std::convert::TryInto;

    let token = identity.token(&elephantry)?;

    let user = match elephantry
        .model::<oxfeed_common::user::Model>()
        .find_from_token(&token)
    {
        Some(user) => user,
        None => return Ok(actix_web::HttpResponse::Unauthorized().finish()),
    };

    data.user_id = Some(user.user_id);
    let source_id = Some(path.into_inner());
    let pk = elephantry::pk!(source_id);
    let source = elephantry.update_one::<Model>(&pk, &data.into_inner().try_into()?)?;

    let response = match source {
        Some(source) => actix_web::HttpResponse::Ok().json(source),
        None => actix_web::HttpResponse::NotFound().finish(),
    };

    Ok(response)
}
