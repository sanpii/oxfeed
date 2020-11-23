pub(crate) mod item;
pub(crate) mod opml;
pub(crate) mod source;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/")
        .service(counts)
}

#[derive(serde::Deserialize)]
pub struct Pagination {
    pub page: usize,
    #[serde(default="default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    25
}

#[derive(elephantry::Entity, serde::Serialize)]
pub struct Counts {
    all: i64,
    favorites: i64,
    sources: i64,
    unread: i64,
}

#[actix_web::get("/counts")]
async fn counts(elephantry: actix_web::web::Data<elephantry::Pool>) -> crate::Result {
    let query = r#"
with
    count_sources as (
        select count(*) from source
    ),
    count_unread as (
        select count(*) from item where not read
    ),
    count_all as (
        select count(*) from item
    ),
    count_favorites as (
        select count(*) from item where favorite
    )
select count_sources.count as sources,
        count_unread.count as unread,
        count_all.count as all,
        count_favorites.count as favorites
    from count_sources,
        count_unread,
        count_all,
        count_favorites;
"#;

    let counts = elephantry.query_one::<Counts>(&query, &[])?;
    let response = actix_web::HttpResponse::Ok().json(counts);

    Ok(response)
}
