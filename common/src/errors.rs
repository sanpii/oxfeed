pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("API error: {0}")]
    Api(String),

    #[error("Auth require")]
    Auth,

    #[cfg(feature = "base64")]
    #[error("{0}")]
    Base64(#[from] base64::DecodeError),

    #[error("Bad request")]
    BadRequest,

    #[cfg(feature = "elephantry")]
    #[error("{0}")]
    Database(#[from] elephantry::Error),

    #[error("{0}")]
    Env(#[from] envir::Error),

    #[cfg(feature = "feed-rs")]
    #[error("{0}")]
    Feed(#[from] feed_rs::parser::ParseFeedError),

    #[cfg(feature = "attohttpc")]
    #[error("{0}")]
    Httpc(#[from] attohttpc::Error),

    #[error("Invalid email or password")]
    InvalidLogin,

    #[error("{0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Jwt(#[from] jwt::Error),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("Not found")]
    NotFound,

    #[cfg(feature = "opml")]
    #[error("{0}")]
    Opml(#[from] opml::Error),

    #[cfg(feature = "reqwest")]
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    #[cfg(feature = "gloo")]
    #[error("Unable to save preference: {0}")]
    Storage(#[from] gloo::storage::errors::StorageError),

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
            Error::BadRequest => StatusCode::BAD_REQUEST,
            Error::InvalidLogin => StatusCode::FORBIDDEN,
            Error::NotFound => StatusCode::NOT_FOUND,
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
            log::error!("{message}");
        }

        actix_web::HttpResponse::build(status).json(message)
    }
}
