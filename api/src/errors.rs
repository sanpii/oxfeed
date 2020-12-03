pub type Result<T = actix_web::HttpResponse> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Auth,
    Base64(base64::DecodeError),
    Database(elephantry::Error),
    Feed(feed_rs::parser::ParseFeedError),
    Http(attohttpc::Error),
    Jwt(jwt::Error),
    Io(std::io::Error),
    Opml(String),
    Utf8(std::string::FromUtf8Error),
    Web(actix_web::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Auth => "missing authentification information".to_string(),
            Self::Base64(error) => error.to_string(),
            Self::Database(error) => error.to_string(),
            Self::Feed(error) => error.to_string(),
            Self::Http(error) => error.to_string(),
            Self::Jwt(error) => error.to_string(),
            Self::Io(error) => error.to_string(),
            Self::Opml(error) => error.to_string(),
            Self::Utf8(error) => error.to_string(),
            Self::Web(error) => error.to_string(),
        };

        write!(f, "{}", message)
    }
}

impl actix_web::ResponseError for Error {}

impl From<actix_web::Error> for Error {
    fn from(error: actix_web::Error) -> Self {
        Self::Web(error)
    }
}

impl From<attohttpc::Error> for Error {
    fn from(error: attohttpc::Error) -> Self {
        Self::Http(error)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(error: base64::DecodeError) -> Self {
        Self::Base64(error)
    }
}

impl From<elephantry::Error> for Error {
    fn from(error: elephantry::Error) -> Self {
        Self::Database(error)
    }
}

impl From<feed_rs::parser::ParseFeedError> for Error {
    fn from(error: feed_rs::parser::ParseFeedError) -> Self {
        Self::Feed(error)
    }
}

impl From<jwt::Error> for Error {
    fn from(error: jwt::Error) -> Self {
        Self::Jwt(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Self::Utf8(error)
    }
}
