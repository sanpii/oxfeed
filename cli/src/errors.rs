pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Database(#[from] elephantry::Error),
    #[error("http error: {0}")]
    Http(#[from] attohttpc::Error),
    #[error("feed parse error: {0}")]
    Parser(#[from] feed_rs::parser::ParseFeedError),
}
