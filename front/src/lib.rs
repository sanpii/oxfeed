#![recursion_limit="512"]

mod components;

use components::*;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
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

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
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

struct Model;

#[derive(yew_router::Switch, Clone)]
enum Route {
    #[to = "/favorites"]
    Favorites,
    #[to = "/sources"]
    Sources,
    #[to = "/unread"]
    Unread,
}

impl yew::Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> yew::Html {
        use yew_router::router::Router;

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
                                render = yew_router::router::Router::render(|switch: Route| {
                                    match switch {
                                        Route::Favorites => yew::html!{<Favorites />},
                                        Route::Sources => yew::html!{<Sources />},
                                        Route::Unread => yew::html!{<Unread />},
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
