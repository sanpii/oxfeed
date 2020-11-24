use std::collections::HashMap;
use crate::model::item::Model;
use actix_web::web::{Data, Json, Path};

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/items")
        .service(content)
        .service(favorites)
        .service(patch)
        .service(unread)
        .service(read_all)
        .service(all)
}

#[actix_web::get("/")]
async fn all(elephantry: Data<elephantry::Pool>, pagination: actix_web::web::Query<super::Pagination>) -> crate::Result {
    fetch(&elephantry, "true", &pagination.into_inner())
}

#[actix_web::get("/favorites")]
async fn favorites(elephantry: Data<elephantry::Pool>, pagination: actix_web::web::Query<super::Pagination>) -> crate::Result {
    fetch(&elephantry, "favorite", &pagination.into_inner())
}

#[actix_web::get("/unread")]
async fn unread(elephantry: Data<elephantry::Pool>, pagination: actix_web::web::Query<super::Pagination>) -> crate::Result {
    fetch(&elephantry, "not read", &pagination.into_inner())
}

fn fetch(elephantry: &elephantry::Pool, filter: &str, pagination: &super::Pagination) -> crate::Result {
    let model = elephantry.model::<Model>();
    let items = model.all(filter, pagination.page, pagination.limit)?;
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

    let response = if !data.is_empty() {
        let source = elephantry.update_by_pk::<Model>(&elephantry::pk!(item_id), &data)?;

        match source {
            Some(source) => actix_web::HttpResponse::Ok().json(source),
            None => actix_web::HttpResponse::NotFound().finish(),
        }
    } else {
        actix_web::HttpResponse::NoContent().finish()
    };

    Ok(response)
}

#[actix_web::post("/read")]
async fn read_all(elephantry: Data<elephantry::Pool>) -> crate::Result {
    elephantry.execute("update item set read = true where not read;")?;

    let response = actix_web::HttpResponse::NoContent().finish();

    Ok(response)
}
