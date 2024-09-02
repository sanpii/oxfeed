use actix_web::web::{Data, Json, Path};
use oxfeed_common::item::Model;
use std::collections::HashMap;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/items")
        .service(content)
        .service(favorites)
        .service(patch)
        .service(unread)
        .service(read_all)
        .service(all)
}

#[actix_web::get("")]
async fn all(
    elephantry: Data<elephantry::Pool>,
    pagination: actix_web::web::Query<elephantry_extras::Pagination>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    fetch(
        &elephantry,
        &identity,
        &elephantry::Where::new(),
        &pagination,
    )
}

#[actix_web::get("/favorites")]
async fn favorites(
    elephantry: Data<elephantry::Pool>,
    pagination: actix_web::web::Query<elephantry_extras::Pagination>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    fetch(
        &elephantry,
        &identity,
        &elephantry::Where::from("favorite", Vec::new()),
        &pagination.into_inner(),
    )
}

#[actix_web::get("/unread")]
async fn unread(
    elephantry: Data<elephantry::Pool>,
    pagination: actix_web::web::Query<elephantry_extras::Pagination>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    fetch(
        &elephantry,
        &identity,
        &elephantry::Where::from("not read", Vec::new()),
        &pagination.into_inner(),
    )
}

pub(crate) fn fetch(
    elephantry: &elephantry::Pool,
    identity: &crate::Identity,
    filter: &elephantry::Where,
    pagination: &elephantry_extras::Pagination,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token(elephantry)?;

    let model = elephantry.model::<Model>();
    let items = model.all(&token, filter, pagination)?;
    let response = actix_web::HttpResponse::Ok().json(items);

    Ok(response)
}

#[actix_web::get("/{item_id}/content")]
async fn content(
    elephantry: Data<elephantry::Pool>,
    path: Path<uuid::Uuid>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let item_id = Some(path.into_inner());
    let sql = include_str!("../../sql/item_content.sql");
    let content = elephantry
        .query::<Option<String>>(sql, &[&item_id, &token])?
        .next()
        .ok_or(oxfeed_common::Error::NotFound)?;

    Ok(actix_web::HttpResponse::Ok().json(content.unwrap_or_default()))
}

#[actix_web::patch("/{item_id}")]
async fn patch(
    elephantry: Data<elephantry::Pool>,
    path: Path<uuid::Uuid>,
    json: Json<serde_json::Value>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let item_id = path.into_inner();

    elephantry
        .model::<Model>()
        .one(&token, &item_id)?
        .ok_or(oxfeed_common::Error::NotFound)?;

    let mut data = HashMap::new();

    for (k, v) in json.as_object().unwrap() {
        let v = match v {
            serde_json::Value::Bool(v) => v as &dyn elephantry::ToSql,
            serde_json::Value::String(v) => v as &dyn elephantry::ToSql,
            _ => todo!(),
        };
        data.insert(k.clone(), v);
    }

    if !data.is_empty() {
        elephantry
            .update_by_pk::<Model>(&elephantry::pk!(item_id), &data)?
            .ok_or(oxfeed_common::Error::NotFound)?;
    }

    Ok(actix_web::HttpResponse::NoContent().finish())
}

#[actix_web::post("/read")]
async fn read_all(
    elephantry: Data<elephantry::Pool>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let sql = include_str!("../../sql/read_all.sql");

    elephantry.query::<()>(sql, &[&token])?;

    let response = actix_web::HttpResponse::NoContent().finish();

    Ok(response)
}
