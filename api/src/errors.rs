pub(crate) type Result<T = actix_web::HttpResponse> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
    Database(elephantry::Error),
    Feed(feed_rs::parser::ParseFeedError),
    Http(attohttpc::Error),
    Opml(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Database(error) => error.to_string(),
            Self::Feed(error) => error.to_string(),
            Self::Http(error) => error.to_string(),
            Self::Opml(error) => error.to_string(),
        };

        write!(f, "{}", message)
    }
}

impl actix_web::ResponseError for Error {}

impl From<attohttpc::Error> for Error {
    fn from(error: attohttpc::Error) -> Self {
        Self::Http(error)
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
