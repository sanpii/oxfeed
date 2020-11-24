use crate::model::source::Model;
use actix_web::web::{Data, Json, Path};

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/sources")
        .service(get)
        .service(delete)
        .service(update)
        .service(all)
        .service(create)
}

#[actix_web::get("")]
async fn all(elephantry: Data<elephantry::Pool>, pagination: actix_web::web::Query<super::Pagination>) -> crate::Result {
    let sources = elephantry.paginate_find_where::<Model>("true", &[], pagination.limit, pagination.page, "order by last_error, title".into())?;
    let response = actix_web::HttpResponse::Ok().json(sources);

    Ok(response)
}

#[actix_web::post("")]
async fn create(
    elephantry: Data<elephantry::Pool>,
    data: Json<crate::form::Source>,
) -> crate::Result {
    use std::convert::TryInto;

    let source = elephantry.insert_one::<Model>(&data.into_inner().try_into()?)?;
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
    use std::convert::TryInto;

    let source_id = Some(path.into_inner());
    let pk = elephantry::pk!(source_id);
    let source = elephantry.update_one::<Model>(&pk, &data.into_inner().try_into()?)?;

    let response = match source {
        Some(source) => actix_web::HttpResponse::Ok().json(source),
        None => actix_web::HttpResponse::NotFound().finish(),
    };

    Ok(response)
}
