use crate::model::item::Model;
use actix_web::web::Data;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/items")
        .service(unread)
}

#[actix_web::get("/unread")]
async fn unread(elephantry: Data<elephantry::Pool>) -> crate::Result {
    let model = elephantry.model::<Model>();
    let items = model.unread()?.collect::<Vec<_>>();
    let response = actix_web::HttpResponse::Ok().json(items);

    Ok(response)
}
