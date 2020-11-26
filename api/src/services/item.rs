use std::collections::HashMap;
use crate::model::item::Model;
use actix_web::web::{Data, Json, Path};

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/items")
        .service(content)
        .service(icon)
        .service(favorites)
        .service(patch)
        .service(unread)
        .service(read_all)
        .service(all)
}

#[actix_web::get("")]
async fn all(elephantry: Data<elephantry::Pool>, pagination: actix_web::web::Query<super::Pagination>) -> crate::Result {
    fetch(&elephantry, &elephantry::Where::new(), &pagination.into_inner())
}

#[actix_web::get("/favorites")]
async fn favorites(elephantry: Data<elephantry::Pool>, pagination: actix_web::web::Query<super::Pagination>) -> crate::Result {
    fetch(&elephantry, &elephantry::Where::from("favorite", &[]), &pagination.into_inner())
}

#[actix_web::get("/unread")]
async fn unread(elephantry: Data<elephantry::Pool>, pagination: actix_web::web::Query<super::Pagination>) -> crate::Result {
    fetch(&elephantry, &elephantry::Where::from("not read", &[]), &pagination.into_inner())
}

pub(crate) fn fetch(elephantry: &elephantry::Pool, filter: &elephantry::Where, pagination: &super::Pagination) -> crate::Result {
    let limit = pagination.limit.parse().unwrap();
    let page = pagination.page.parse().unwrap();
    let model = elephantry.model::<Model>();
    let items = model.all(filter, page, limit)?;
    let response = actix_web::HttpResponse::Ok().json(items);

    Ok(response)
}

#[actix_web::get("/{item_id}/content")]
async fn content(elephantry: Data<elephantry::Pool>, path: Path<uuid::Uuid>) -> crate::Result {
    let item_id = Some(path.into_inner());
    let content = elephantry.query::<Option<String>>("select content from item where item_id = $*", &[&item_id])?.next();
    let response = match content {
        Some(content) => actix_web::HttpResponse::Ok().body(&content.unwrap_or_default()),
        None => actix_web::HttpResponse::NotFound().finish(),
    };

    Ok(response)
}

#[actix_web::get("/{item_id}/icon")]
async fn icon(elephantry: Data<elephantry::Pool>, path: Path<uuid::Uuid>) -> crate::Result {
    let empty_img = [
        71, 73, 70, 56, 57, 97, 1, 0, 1, 0, 128, 0, 0, 255, 255, 255, 255, 255,
        255, 33, 249, 4, 1, 10, 0, 1, 0, 44, 0, 0, 0, 0, 1, 0, 1, 0, 0, 2, 2,
        76, 1, 0, 59
    ];

    let item_id = path.into_inner();
    let icon = elephantry.query_one::<Option<String>>("select icon from item where item_id = $*", &[&item_id])?;
    let mut img = None;

    if let Some(icon) = icon {
        img = crate::cache::get(&icon).ok();
    }

    let body = img.unwrap_or(empty_img.to_vec());

    let mut mime = tree_magic::from_u8(&body);
    if mime == "text/plain" {
        mime = "image/svg+xml".to_string();
    }

    let response = actix_web::HttpResponse::Ok()
        .header("Content-Type", mime)
        .body(body);

    Ok(response)
}

#[actix_web::patch("/{item_id}")]
async fn patch(
    elephantry: Data<elephantry::Pool>,
    path: Path<uuid::Uuid>,
    json: Json<serde_json::Value>,
) -> crate::Result {
    let item_id = path.into_inner();
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
async fn read_all(elephantry: Data<elephantry::Pool>) -> crate::Result {
    elephantry.execute("update item set read = true where not read;")?;

    let response = actix_web::HttpResponse::NoContent().finish();

    Ok(response)
}
