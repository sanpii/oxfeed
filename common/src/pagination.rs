#[derive(Clone, Copy, Eq, PartialEq, serde::Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_page() -> usize {
    1
}

fn default_limit() -> usize {
    25
}

impl Pagination {
    pub fn new() -> Self {
        Self {
            page: default_page(),
            limit: default_limit(),
        }
    }

    pub fn to_sql(&self) -> String {
        format!(
            "offset {} fetch first {} rows only",
            (self.page - 1) * self.limit,
            self.limit,
        )
    }

    pub fn to_query(&self) -> String {
        format!("page={}&limit={}", self.page, self.limit)
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new()
    }
}
