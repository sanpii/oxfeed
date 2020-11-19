use crate::model::item::Model;
use actix_web::web::{Data, Path};

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/items")
        .service(unread)
        .service(content)
}

#[actix_web::get("/unread")]
async fn unread(elephantry: Data<elephantry::Pool>) -> crate::Result {
    let model = elephantry.model::<Model>();
    let items = model.unread()?.collect::<Vec<_>>();
    let response = actix_web::HttpResponse::Ok().json(items);

    Ok(response)
}

#[actix_web::get("/{item_id}/content")]
async fn content(elephantry: Data<elephantry::Pool>, path: Path<uuid::Uuid>) -> crate::Result {
    let item_id = Some(path.into_inner());
    let content = elephantry.query::<String>("select content from item where item_id = $*", &[&item_id])?.next();
    let response = match content {
        Some(content) => actix_web::HttpResponse::Ok().body(&content),
        None => actix_web::HttpResponse::NotFound().finish(),
    };

    Ok(response)
}
