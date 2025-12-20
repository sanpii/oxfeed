pub mod account;
pub mod filter;
pub mod item;
pub mod media;
pub mod source;
pub mod user;
pub mod webhook;

mod errors;
mod tag;

pub use errors::*;
pub use tag::*;

#[derive(Debug, Clone, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
pub struct Counts {
    pub all: i64,
    pub favorites: i64,
    pub sources_has_error: bool,
    pub sources: i64,
    pub tags: i64,
    pub unread: i64,
}
