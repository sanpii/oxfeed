mod errors;
pub mod item;
pub mod new_user;
pub mod source;
pub mod user;

mod pagination;

pub use errors::*;
pub use pagination::*;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
pub struct Counts {
    pub all: i64,
    pub favorites: i64,
    pub sources: i64,
    pub unread: i64,
}
