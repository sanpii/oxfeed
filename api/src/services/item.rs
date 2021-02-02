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
    pagination: actix_web::web::Query<oxfeed_common::Pagination>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    fetch(
        &elephantry,
        &identity,
        &elephantry::Where::new(),
        &pagination.into_inner(),
    )
}

#[actix_web::get("/favorites")]
async fn favorites(
    elephantry: Data<elephantry::Pool>,
    pagination: actix_web::web::Query<oxfeed_common::Pagination>,
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
    pagination: actix_web::web::Query<oxfeed_common::Pagination>,
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
    pagination: &oxfeed_common::Pagination,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token();

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
    let token = identity.token();
    let item_id = Some(path.into_inner());
    let sql = include_str!("../../sql/item_content.sql");
    let content = elephantry
        .query::<Option<String>>(sql, &[&item_id, &token])?
        .next();
    let response = match content {
        Some(content) => actix_web::HttpResponse::Ok().body(&content.unwrap_or_default()),
        None => actix_web::HttpResponse::NotFound().finish(),
    };

    Ok(response)
}

#[actix_web::patch("/{item_id}")]
async fn patch(
    elephantry: Data<elephantry::Pool>,
    path: Path<uuid::Uuid>,
    json: Json<serde_json::Value>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token();
    let item_id = path.into_inner();

    match elephantry.model::<Model>().one(&token, &item_id)? {
        Some(_) => (),
        None => return Ok(actix_web::HttpResponse::NotFound().finish()),
    }

    let mut data = HashMap::new();

    for (k, v) in json.as_object().unwrap() {
        let v = match v {
            serde_json::Value::Bool(v) => v as &dyn elephantry::ToSql,
            serde_json::Value::String(v) => v as &dyn elephantry::ToSql,
            _ => todo!(),
        };
        data.insert(k.clone(), v);
    }

    let mut response = if !data.is_empty() {
        let item = elephantry.update_by_pk::<Model>(&elephantry::pk!(item_id), &data)?;

        match item {
            Some(_) => actix_web::HttpResponse::NoContent(),
            None => actix_web::HttpResponse::NotFound(),
        }
    } else {
        actix_web::HttpResponse::NoContent()
    };

    Ok(response.finish())
}

#[actix_web::post("/read")]
async fn read_all(
    elephantry: Data<elephantry::Pool>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token();
    let sql = include_str!("../../sql/read_all.sql");

    elephantry.query::<()>(sql, &[&token])?;

    let response = actix_web::HttpResponse::NoContent().finish();

    Ok(response)
}
