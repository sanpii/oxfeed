pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Auth require")]
    Auth,

    #[error("{0}")]
    Any(#[from] anyhow::Error),

    #[cfg(feature = "base64")]
    #[error("{0}")]
    Base64(#[from] base64::DecodeError),

    #[cfg(feature = "elephantry")]
    #[error("{0}")]
    Database(#[from] elephantry::Error),

    #[cfg(feature = "feed-rs")]
    #[error("{0}")]
    Feed(#[from] feed_rs::parser::ParseFeedError),

    #[cfg(feature = "attohttpc")]
    #[error("{0}")]
    Httpc(#[from] attohttpc::Error),

    #[error("{0}")]
    Http(#[from] http::Error),

    #[error("{0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Jwt(#[from] jwt::Error),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "opml")]
    #[error("{0}")]
    Opml(#[from] opml::Error),

    #[error("{0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[cfg(feature = "actix-web")]
    #[error("{0}")]
    Web(#[from] actix_web::Error),

    #[error("{0}")]
    Webhook(String),
}

#[cfg(feature = "actix-web")]
impl From<&Error> for actix_web::http::StatusCode {
    fn from(error: &Error) -> Self {
        use actix_web::http::StatusCode;

        match error {
            Error::Auth => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "actix-web")]
impl actix_web::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        let status: actix_web::http::StatusCode = self.into();

        let message = self.to_string();

        if status.is_server_error() {
            log::error!("{}", message);
        }

        actix_web::HttpResponse::build(status).json(message)
    }
}
