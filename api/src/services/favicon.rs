pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/favicon")
        .service(default)
        .service(error)
        .service(unread)
}

#[actix_web::get("default.svg")]
async fn default() -> actix_web::HttpResponse {
    favicon("black", None)
}

#[actix_web::get("error.svg")]
async fn error() -> actix_web::HttpResponse {
    favicon("#dc3545", None)
}

#[actix_web::get("unread-{nb}.svg")]
async fn unread(nb: actix_web::web::Path<usize>) -> actix_web::HttpResponse {
    favicon("#0d6efd", Some(*nb))
}

fn favicon(color: &str, nb: Option<usize>) -> actix_web::HttpResponse {
    let badge = match nb {
        Some(nb) if nb < 10 => format!(
            r#"<circle cx="11" cy="11" r="5" fill="red" /><text x="8" y="14" fill="white" font-size="9" font-weight="900">{nb}</text>"#,
        ),
        Some(nb) => format!(
            r#"<ellipse cx="11" cy="11" rx="7" ry="5" fill="red" /><text x="4" y="14" fill="white" font-size="9" font-weight="900">{nb}</text>"#,
        ),
        None => String::new(),
    };

    let icon = format!(
        r#"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="{color}" class="bi bi-rss" viewBox="0 0 16 16">
    <path d="M14 1a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H2a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1zM2 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2z"/>
    <path d="M5.5 12a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0m-3-8.5a1 1 0 0 1 1-1c5.523 0 10 4.477 10 10a1 1 0 1 1-2 0 8 8 0 0 0-8-8 1 1 0 0 1-1-1m0 4a1 1 0 0 1 1-1 6 6 0 0 1 6 6 1 1 0 1 1-2 0 4 4 0 0 0-4-4 1 1 0 0 1-1-1"/>
    {badge}
</svg>
"#
    );

    actix_web::HttpResponse::Ok()
        .append_header(("Content-Type", "image/svg+xml"))
        .append_header(("Cache-Control", "public, max-age=604800"))
        .body(icon)
}
