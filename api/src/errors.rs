pub(crate) type Result<T = actix_web::HttpResponse> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
    Database(elephantry::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Database(error) => error,
        };

        write!(f, "{}", message)
    }
}

impl actix_web::ResponseError for Error {}

impl From<elephantry::Error> for Error {
    fn from(error: elephantry::Error) -> Self {
        Self::Database(error)
    }
}
