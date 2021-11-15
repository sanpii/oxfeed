const EMPTY_IMG: [u8; 43] = [
    71, 73, 70, 56, 57, 97, 1, 0, 1, 0, 128, 0, 0, 255, 255, 255, 255, 255, 255, 33, 249, 4, 1, 10,
    0, 1, 0, 44, 0, 0, 0, 0, 1, 0, 1, 0, 0, 2, 2, 76, 1, 0, 59,
];

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/icons").service(icon)
}

#[actix_web::get("/{url:.*}")]
async fn icon(url: actix_web::web::Path<String>) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let url = base64::decode(url.into_inner())?;
    let icon = String::from_utf8(url)?;

    let body = match crate::cache::get(&icon) {
        Ok(body) => body,
        Err(_) => EMPTY_IMG.to_vec(),
    };

    let mut mime = tree_magic::from_u8(&body);
    if mime == "text/plain" {
        mime = "image/svg+xml".to_string();
    }

    let response = actix_web::HttpResponse::Ok()
        .append_header(("Content-Type", mime))
        .append_header(("Cache-Control", "public, max-age=604800, immutable"))
        .body(body);

    Ok(response)
}
