pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("{0}")]
    Any(#[from] anyhow::Error),
    #[error("API response error: {0}")]
    Api(#[from] serde_json::Error),
    #[error("API response error: {0}")]
    Http(#[from] http::Error),
}
