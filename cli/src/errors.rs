pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Database(#[from] elephantry::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("http error: {0}")]
    Http(#[from] attohttpc::Error),
    #[error("{0}")]
    Parser(#[from] feed_rs::parser::ParseFeedError),
}
