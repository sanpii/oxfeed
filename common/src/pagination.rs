#[derive(Clone, Copy, Eq, PartialEq, serde::Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page", deserialize_with = "parse")]
    pub page: usize,
    #[serde(default = "default_limit", deserialize_with = "parse")]
    pub limit: usize,
}

fn default_page() -> usize {
    1
}

fn default_limit() -> usize {
    25
}

fn parse<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde::Deserialize;

    let s = String::deserialize(deserializer)?;

    s.parse().map_err(D::Error::custom)
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
