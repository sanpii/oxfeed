use oxfeed::webhook::Model;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/webhooks")
        .service(all)
        .service(create)
        .service(delete)
        .service(execute)
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
    mut data: actix_web::web::Json<crate::form::Webhook>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    use std::convert::TryInto;

    let token = identity.token(&elephantry)?;

    let user = elephantry
        .model::<oxfeed::user::Model>()
        .find_from_token(&token)
        .ok_or(oxfeed::Error::Auth)?;

    data.user_id = Some(user.id);
    let webhook = elephantry.insert_one::<Model>(&data.into_inner().try_into()?)?;
    let response = actix_web::HttpResponse::Ok().json(webhook);

    Ok(response)
}

#[actix_web::delete("/{webhook_id}")]
async fn delete(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    path: actix_web::web::Path<uuid::Uuid>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let webhook_id = path.into_inner();
    let token = identity.token(&elephantry)?;
    let response = match elephantry.model::<Model>().delete(&token, &webhook_id)? {
        Some(webhook) => actix_web::HttpResponse::Ok().json(webhook),
        None => actix_web::HttpResponse::NoContent().finish(),
    };

    Ok(response)
}

#[actix_web::post("/{webhook_id}")]
async fn execute(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    path: actix_web::web::Path<uuid::Uuid>,
    item: actix_web::web::Json<oxfeed::item::Item>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let webhook_id = path.into_inner();
    let token = identity.token(&elephantry)?;
    let Some(webhook) = elephantry.model::<Model>().one(&token, &webhook_id)? else {
        return Err(oxfeed::Error::BadRequest);
    };

    let Some(item) = elephantry
        .model::<oxfeed::item::Model>()
        .one(&token, &item.id)?
    else {
        return Err(oxfeed::Error::BadRequest);
    };

    let response = match crate::execute_webhook(&webhook, &item).await {
        Ok(body) => oxfeed::webhook::Response {
            status: reqwest::StatusCode::OK,
            body,
        },
        Err(oxfeed::Error::Webhook(status, body)) => oxfeed::webhook::Response { status, body },
        Err(err) => return Err(err),
    };

    let response = actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::OK)
        .append_header(actix_web::http::header::ContentType(mime::TEXT_PLAIN))
        .json(response);

    Ok(response)
}

#[actix_web::put("/{source_id}")]
async fn update(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    mut data: actix_web::web::Json<crate::form::Webhook>,
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
    let webhook_id = Some(path.into_inner());
    let pk = elephantry::pk!(webhook_id);
    let webhook = elephantry
        .update_one::<Model>(&pk, &data.into_inner().try_into()?)?
        .ok_or(oxfeed::Error::NotFound)?;

    Ok(actix_web::HttpResponse::Ok().json(webhook))
}
