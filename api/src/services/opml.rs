use actix_web::web::Data;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/opml")
        .service(export)
        .service(import)
}

#[actix_web::post("")]
async fn import(
    elephantry: Data<elephantry::Pool>,
    xml: actix_web::web::Json<String>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;

    let user = elephantry
        .model::<oxfeed::user::Model>()
        .find_from_token(&token)
        .ok_or(oxfeed::Error::Auth)?;

    let opml = opml::OPML::from_str(&xml.0).unwrap();

    for outline in opml.body.outlines {
        save(&elephantry, &outline, &user);
    }

    let response = actix_web::HttpResponse::NoContent().finish();

    Ok(response)
}

fn save(elephantry: &elephantry::Pool, outline: &opml::Outline, user: &oxfeed::user::Entity) {
    for outline in &outline.outlines {
        save(elephantry, outline, user);
    }

    let Some(source) = source_try_from(outline, user) else {
        return;
    };

    if let Err(error) = elephantry.insert_one::<oxfeed::source::Model>(&source) {
        log::error!("Unable to import outline '{}': {error}", source.title);
    }
}

fn source_try_from(
    outline: &opml::Outline,
    user: &oxfeed::user::Entity,
) -> Option<oxfeed::source::Entity> {
    let url = match &outline.xml_url {
        Some(url) => url.clone(),
        None => return None,
    };

    let mut tags = Vec::new();

    if let Some(category) = &outline.category {
        tags.push(category.clone());
    }

    let entity = oxfeed::source::Entity {
        tags,
        title: outline.text.clone(),
        url,
        user_id: user.id,

        ..Default::default()
    };

    Some(entity)
}

#[actix_web::get("")]
async fn export(elephantry: Data<elephantry::Pool>) -> oxfeed::Result<actix_web::HttpResponse> {
    let mut opml = opml::OPML::default();

    let feeds = elephantry.query::<(String, String)>("select (title, url) from source", &[])?;

    for (title, url) in feeds {
        opml.add_feed(&title, &url);
    }

    let response = actix_web::HttpResponse::Ok()
        .append_header(("Content-Type", "text/xml; charset=utf-8"))
        .append_header((
            "Content-Disposition",
            "attachment; filename=\"oxfeed-subscriptions.xml\"",
        ))
        .body(opml.to_string()?);

    Ok(response)
}
