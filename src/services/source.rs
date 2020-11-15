use crate::model::source::Model;
use actix_web::web::{Data, Json, Path};

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/sources")
        .service(all)
        .service(create)
        .service(get)
        .service(delete)
        .service(update)
}

#[actix_web::get("/")]
async fn all(elephantry: Data<elephantry::Pool>) -> crate::Result {
    let sources = elephantry.find_all::<Model>(None)?.collect::<Vec<_>>();
    let response = actix_web::HttpResponse::Ok().json(sources);

    Ok(response)
}

#[actix_web::post("/")]
async fn create(
    elephantry: Data<elephantry::Pool>,
    data: Json<crate::form::Source>,
) -> crate::Result {
    let source = elephantry.insert_one::<Model>(&data.into_inner().into())?;
    let response = actix_web::HttpResponse::Ok().json(source);

    Ok(response)
}

#[actix_web::get("/{source_id}")]
async fn get(elephantry: Data<elephantry::Pool>, path: Path<uuid::Uuid>) -> crate::Result {
    let source_id = Some(path.into_inner());
    let pk = elephantry::pk!(source_id);
    let source = elephantry.find_by_pk::<Model>(&pk)?;

    let response = match source {
        Some(source) => actix_web::HttpResponse::Ok().json(source),
        None => actix_web::HttpResponse::NotFound().finish(),
    };

    Ok(response)
}

#[actix_web::delete("/{source_id}")]
async fn delete(elephantry: Data<elephantry::Pool>, path: Path<uuid::Uuid>) -> crate::Result {
    let source_id = Some(path.into_inner());

    elephantry.delete_where::<crate::model::item::Model>(
        "source_id = $*", &[&source_id]
    )?;

    let pk = elephantry::pk!(source_id);
    let source = elephantry.delete_by_pk::<Model>(&pk)?;

    let response = match source {
        Some(source) => actix_web::HttpResponse::Ok().json(source),
        None => actix_web::HttpResponse::NoContent().finish(),
    };

    Ok(response)
}

#[actix_web::put("/{source_id}")]
async fn update(
    elephantry: Data<elephantry::Pool>,
    data: Json<crate::form::Source>,
    path: Path<uuid::Uuid>,
) -> crate::Result {
    let source_id = Some(path.into_inner());
    let pk = elephantry::pk!(source_id);
    let source = elephantry.update_one::<Model>(&pk, &data.into_inner().into())?;

    let response = match source {
        Some(source) => actix_web::HttpResponse::Ok().json(source),
        None => actix_web::HttpResponse::NotFound().finish(),
    };

    Ok(response)
}
