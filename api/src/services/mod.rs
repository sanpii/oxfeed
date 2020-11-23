#[derive(serde::Deserialize)]
pub struct Pagination {
    pub page: usize,
    #[serde(default="default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    25
}

pub(crate) mod item;
pub(crate) mod source;
