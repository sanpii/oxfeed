pub(crate) mod auth;
pub(crate) mod item;
pub(crate) mod opml;
pub(crate) mod search;
pub(crate) mod source;
pub(crate) mod user;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/")
        .service(counts)
}

#[derive(serde::Deserialize)]
pub struct Pagination {
    #[serde(default="default_page")]
    pub page: String,
    #[serde(default="default_limit")]
    pub limit: String,
}

fn default_page() -> String {
    "1".to_string()
}

fn default_limit() -> String {
    "25".to_string()
}

impl Pagination {
    pub fn to_sql(&self) -> String {
        format!(
            "offset {} fetch first {} rows only",
            (self.page() - 1) * self.limit(),
            self.limit(),
        )
    }

    pub fn page(&self) -> usize {
        self.page.parse().unwrap()
    }

    pub fn limit(&self) -> usize {
        self.limit.parse().unwrap()
    }
}

#[derive(elephantry::Entity, serde::Serialize)]
pub struct Counts {
    all: i64,
    favorites: i64,
    sources: i64,
    unread: i64,
}

#[actix_web::get("/counts")]
async fn counts(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    identity: crate::Identity,
) -> crate::Result {
    let token = match identity.token() {
        Some(token) => token,
        None => return Ok(actix_web::HttpResponse::Unauthorized().finish()),
    };

    let sql = include_str!("../sql/counts.sql");
    let counts = elephantry.query_one::<Counts>(sql, &[&token])?;
    let response = actix_web::HttpResponse::Ok().json(counts);

    Ok(response)
}
