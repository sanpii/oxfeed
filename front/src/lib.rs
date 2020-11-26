#![recursion_limit="1024"]

mod api;
mod cha;
mod components;
mod errors;
mod event;
mod location;

pub(crate) use api::Api;
pub(crate) use errors::*;
pub(crate) use location::Location;

trait Render: Clone + Eq + PartialEq {
    fn render(&self) -> yew::Html;
}

impl Render for String {
    fn render(&self) -> yew::Html {
        self.into()
    }
}

#[derive(Clone, Eq, PartialEq, serde::Deserialize)]
struct Pager<R: Render> {
    result_count: usize,
    result_min: usize,
    result_max: usize,
    last_page: usize,
    page: usize,
    has_next_page: bool,
    has_previous_page: bool,
    count: usize,
    max_per_page: usize,
    #[serde(default="location::base_url")]
    base_url: String,
    iterator: Vec<R>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct Source {
    last_error: Option<String>,
    source_id: Option<String>,
    tags: Vec<String>,
    title: Option<String>,
    url: String,
}

impl Into<std::result::Result<std::string::String, anyhow::Error>> for &Source {
    fn into(self) -> std::result::Result<std::string::String, anyhow::Error> {
        let json = serde_json::to_string(self)?;

        Ok(json)
    }
}

impl Render for Source {
    fn render(&self) -> yew::Html {
        yew::html! {
            <components::Source value=self />
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct Item {
    item_id: String,
    icon: Option<String>,
    link: String,
    title: String,
    published: chrono::DateTime<chrono::Utc>,
    source: String,
    read: bool,
    favorite: bool,
    tags: Vec<String>,
}

impl Into<std::result::Result<std::string::String, anyhow::Error>> for &Item {
    fn into(self) -> std::result::Result<std::string::String, anyhow::Error> {
        let json = serde_json::to_string(self)?;

        Ok(json)
    }
}

impl Render for Item {
    fn render(&self) -> yew::Html {
        yew::html! {
            <components::Item value=self />
        }
    }
}

#[derive(Clone, serde::Deserialize)]
struct Counts {
    all: usize,
    favorites: usize,
    sources: usize,
    unread: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pagination {
    page: usize,
    limit: usize,
}

impl Pagination {
    fn new() -> Self {
        Self {
            page: 1,
            limit: 25,
        }
    }
}

impl From<Location> for Pagination {
    fn from(location: Location) -> Self {
        let query = location.query();

        Pagination {
            page: query.get("page").map(|x| x.parse().ok()).flatten().unwrap_or(1),
            limit: query.get("limit").map(|x| x.parse().ok()).flatten().unwrap_or(25),
        }
    }
}

struct App;

impl yew::Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <components::App />
        }
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    /*
    let level = if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Warn
    };
    */

    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::initialize();
    yew::App::<App>::new().mount_to_body();
}
