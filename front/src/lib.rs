#![recursion_limit="512"]

mod components;

use components::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct Source {
    source_id: Option<String>,
    title: Option<String>,
    url: String,
}

impl Into<Result<std::string::String, anyhow::Error>> for &Source {
    fn into(self) -> Result<std::string::String, anyhow::Error> {
        let json = serde_json::to_string(self)?;

        Ok(json)
    }
}

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
}

impl Into<Result<std::string::String, anyhow::Error>> for &Item {
    fn into(self) -> Result<std::string::String, anyhow::Error> {
        let json = serde_json::to_string(self)?;

        Ok(json)
    }
}

#[derive(yew_router::Switch, Clone)]
enum Route {
    #[to = "/favorites"]
    Favorites,
    #[to = "/sources"]
    Sources,
    #[to = "/unread"]
    Unread,
    #[to = "/"]
    All,
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

struct Model {
    pagination: Pagination,
}

impl yew::Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        let router = yew_router::service::RouteService::<()>::new();
        let pagination = router.get_query().trim_start_matches('?').parse().unwrap();

        Self {
            pagination,
        }
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> yew::Html {
        use yew_router::router::Router;

        let pagination = self.pagination;

        yew::html! {
            <>
                <nav class="navbar navbar-dark sticky-top bg-dark flex-md-nowrap p-0 shadow">
                    <Header />
                </nav>
                <div class="container-fluid">
                    <div class="row">
                        <nav id="sidebarMenu" class="col-md-3 col-lg-2 d-md-block bg-light sidebar collapse">
                            <Sidebar />
                        </nav>
                        <main class="col-md-9 ml-sm-auto col-lg-10">
                            <Router<Route, ()>
                                render = yew_router::router::Router::render(move |switch: Route| {
                                    match switch {
                                        Route::All => yew::html!{<All pagination=pagination />},
                                        Route::Favorites => yew::html!{<Favorites pagination=pagination />},
                                        Route::Sources => yew::html!{<Sources pagination=pagination />},
                                        Route::Unread => yew::html!{<Unread pagination=pagination />},
                                    }
                                })
                            />
                        </main>
                    </div>
                </div>
            </>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
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
    yew::App::<Model>::new().mount_to_body();
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
                Err(_) => Vec::new(),
            }
        },
    );

    let fetch_task = yew::services::FetchService::fetch(request, callback)?;

    Ok(fetch_task)
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
        {
            fetch(&stringify!($method).to_uppercase(), link, url, body)
        }
    };
}

decl_fetch!(delete);
decl_fetch!(get);
decl_fetch!(patch);
decl_fetch!(post);
decl_fetch!(put);
