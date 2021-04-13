#[derive(Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
pub struct Tag {
    pub name: String,
    pub count: i64,
}
