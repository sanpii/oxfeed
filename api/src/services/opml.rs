use actix_web::web::Data;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/opml")
        .service(export)
        .service(import)
}

#[actix_web::post("")]
async fn import(
    elephantry: Data<elephantry::Pool>,
    xml: String,
    identity: crate::Identity,
) -> crate::Result {
    let user = match elephantry
        .model::<crate::model::user::Model>()
        .find_from_identity(&identity)
    {
        Some(user) => user,
        None => return Ok(actix_web::HttpResponse::Unauthorized().finish()),
    };

    let opml = opml::OPML::new(&xml).unwrap();

    for outline in opml.body.outlines {
        save(&elephantry, &outline, &user);
    }

    let response = actix_web::HttpResponse::NoContent().finish();

    Ok(response)
}

fn save(elephantry: &elephantry::Pool, outline: &opml::Outline, user: &crate::model::user::Entity) {
    use std::convert::TryInto;

    for outline in &outline.outlines {
        save(&elephantry, outline, user);
    }

    let source = match (outline, user).try_into() {
        Ok(source) => source,
        Err(_) => return,
    };

    if let Err(error) = elephantry.insert_one::<crate::model::source::Model>(&source) {
        log::error!("Unable to import outline '{}': {}", source.title, error);
    }
}

#[actix_web::get("")]
async fn export(elephantry: Data<elephantry::Pool>) -> crate::Result {
    let mut opml = opml::OPML::default();

    let feeds = elephantry.query::<(String, String)>("select (title, url) from source", &[])?;

    for (title, url) in feeds {
        opml.add_feed(&title, &url);
    }

    let response = actix_web::HttpResponse::Ok()
        .header("Content-Type", "text/xml; charset=utf-8")
        .header(
            "Content-Disposition",
            "attachment; filename=\"oxfeed-subscriptions.xml\"",
        )
        .body(opml.to_xml().map_err(|e| crate::Error::Opml(e))?);

    Ok(response)
}
