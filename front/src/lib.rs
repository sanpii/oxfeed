#![recursion_limit="1024"]

mod cha;
mod components;
mod event;

#[derive(Clone, Eq, PartialEq, serde::Deserialize)]
struct Pager<T: Clone + Eq + PartialEq> {
    result_count: usize,
    result_min: usize,
    result_max: usize,
    last_page: usize,
    page: usize,
    has_next_page: bool,
    has_previous_page: bool,
    count: usize,
    max_per_page: usize,
    iterator: Vec<T>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct Source {
    last_error: Option<String>,
    source_id: Option<String>,
    tags: Vec<String>,
    title: Option<String>,
    url: String,
}

impl Into<Result<std::string::String, anyhow::Error>> for &Source {
    fn into(self) -> Result<std::string::String, anyhow::Error> {
        let json = serde_json::to_string(self)?;

        Ok(json)
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

impl Into<Result<std::string::String, anyhow::Error>> for &Item {
    fn into(self) -> Result<std::string::String, anyhow::Error> {
        let json = serde_json::to_string(self)?;

        Ok(json)
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

impl std::str::FromStr for Pagination {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pagination = Pagination {
            page: 1,
            limit: 25,
        };

        for args in s.split('&') {
            let tokens = args.split("=").collect::<Vec<_>>();

            match tokens[0] {
                "page" => pagination.page = tokens[1].parse()?,
                "limit" => pagination.limit = tokens[1].parse()?,
                _ => continue,
            }
        }

        Ok(pagination)
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

macro_rules! decl_fetch {
    ($method:ident) => {
        pub(crate) fn $method<B, C>(
            link: &yew::ComponentLink<C>,
            url: &str,
            body: B,
        ) -> Result<yew::services::fetch::FetchTask, Box<dyn std::error::Error>>
        where
            B: Into<Result<String, anyhow::Error>>,
            C: yew::Component,
            <C as yew::Component>::Message: std::convert::TryFrom<(http::Method, yew::format::Text)> + Clone,
            <<C as yew::Component>::Message as std::convert::TryFrom<(http::Method, std::result::Result<std::string::String, anyhow::Error>)>>::Error: std::fmt::Debug,
        {
            fetch(&stringify!($method).to_uppercase(), link, url, body)
        }
    };
}

pub(crate) fn fetch<B, C>(
    method: &str,
    link: &yew::ComponentLink<C>,
    url: &str,
    body: B,
) -> Result<yew::services::fetch::FetchTask, Box<dyn std::error::Error>>
where
    B: Into<Result<String, anyhow::Error>>,
    C: yew::Component,
    <C as yew::Component>::Message: std::convert::TryFrom<(http::Method, yew::format::Text)> + Clone,
    <<C as yew::Component>::Message as std::convert::TryFrom<(http::Method, std::result::Result<std::string::String, anyhow::Error>)>>::Error: std::fmt::Debug,
{
    let request = yew::services::fetch::Request::builder()
        .method(method)
        .uri(&format!("{}{}", env!("API_URL"), url))
        .header("Content-Type", "application/json")
        .body(body)?;

    let method = request.method().clone();

    let callback = link.batch_callback(
        move |response: yew::services::fetch::Response<yew::format::Text>| {
            use std::convert::TryFrom;

            match <C as yew::Component>::Message::try_from((method.clone(), response.into_body())) {
                Ok(message) => vec![message],
                Err(err) => {
                    log::error!("{:?}", err);
                    Vec::new()
                },
            }
        },
    );

    let fetch_task = yew::services::FetchService::fetch(request, callback)?;

    Ok(fetch_task)
}

decl_fetch!(delete);
decl_fetch!(get);
decl_fetch!(patch);
decl_fetch!(post);
decl_fetch!(put);
