use actix_web::web::Data;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/opml")
        .service(export)
}

#[actix_web::get("")]
async fn export(elephantry: Data<elephantry::Pool>) -> crate::Result {
    let mut opml = opml::OPML::default();

    let feeds = elephantry.query::<(String, String)>(r#"
select (title, url) from source;
"#, &[])?;

    for (title, url) in feeds {
        opml.add_feed(&title, &url);
    }

    let response = actix_web::HttpResponse::Ok()
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("Content-Disposition", "attachment; filename=\"oxfeed-subscriptions.xml\"")
        .body(opml.to_xml().map_err(|e| crate::Error::Opml(e))?);

    Ok(response)
}
