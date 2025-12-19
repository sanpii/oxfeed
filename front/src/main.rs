#![warn(warnings)]

mod action;
mod alert;
mod api;
mod cha;
mod context;
mod filter;
mod location;

pub(crate) mod components;

pub(crate) use action::Action;
pub(crate) use alert::Alert;
pub(crate) use api::Api;
pub(crate) use context::Context;
pub(crate) use filter::*;
pub(crate) use location::Location;

#[yew::hook]
pub(crate) fn use_context() -> yew::UseReducerHandle<Context> {
    yew::use_context().unwrap()
}

#[derive(Clone, Eq, PartialEq, serde::Deserialize)]
pub(crate) struct Pager<T> {
    result_count: usize,
    result_min: usize,
    result_max: usize,
    last_page: usize,
    page: usize,
    has_next_page: bool,
    has_previous_page: bool,
    count: usize,
    max_per_page: usize,
    #[serde(default = "location::base_url")]
    base_url: String,
    iterator: Vec<T>,
}

impl<T> Default for Pager<T> {
    fn default() -> Self {
        Self {
            result_count: Default::default(),
            result_min: Default::default(),
            result_max: Default::default(),
            last_page: Default::default(),
            page: Default::default(),
            has_next_page: Default::default(),
            has_previous_page: Default::default(),
            count: Default::default(),
            max_per_page: Default::default(),
            base_url: Default::default(),
            iterator: Default::default(),
        }
    }
}

impl<T> Pager<T> {
    fn is_empty(&self) -> bool {
        self.result_count == 0
    }
}

impl<T> From<Pager<T>> for elephantry_extras::Pager {
    fn from(pager: Pager<T>) -> Self {
        elephantry_extras::Pager {
            count: pager.count,
            page: pager.page,
            max_per_page: pager.max_per_page,
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    yew::Renderer::<components::App>::new().render();
}
