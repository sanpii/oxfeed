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

struct Model;

#[derive(yew_router::Switch, Clone)]
enum Route {
    #[to = "/sources"]
    Sources,
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
                                        Route::Sources => yew::html!{<Sources />},
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
    let level = if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Warn
    };

    wasm_logger::init(wasm_logger::Config::new(level));
    yew::initialize();
    yew::App::<Model>::new().mount_to_body();
}

pub(crate) fn fetch<B, C>(
    method: &str,
    link: &yew::ComponentLink<C>,
    url: &str,
    body: B,
    message: <C as yew::Component>::Message,
) -> Result<yew::services::fetch::FetchTask, Box<dyn std::error::Error>>
where
    B: Into<Result<String, anyhow::Error>>,
    C: yew::Component,
    <C as yew::Component>::Message: std::convert::TryFrom<yew::format::Text> + Clone,
{
    let request = yew::services::fetch::Request::builder()
        .method(method)
        .uri(&format!("{}{}", env!("API_URL"), url))
        .header("Content-Type", "application/json")
        .body(body)?;

    let callback = link.callback(
        move |response: yew::services::fetch::Response<yew::format::Text>| {
            use std::convert::TryFrom;

            <C as yew::Component>::Message::try_from(response.into_body())
                .unwrap_or_else(|_| message.clone())
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
            message: <C as yew::Component>::Message,
        ) -> Result<yew::services::fetch::FetchTask, Box<dyn std::error::Error>>
        where
            B: Into<Result<String, anyhow::Error>>,
            C: yew::Component,
            <C as yew::Component>::Message: std::convert::TryFrom<yew::format::Text> + Clone,
        {
            fetch(stringify!($method), link, url, body, message)
        }
    };
}

decl_fetch!(delete);
decl_fetch!(get);
decl_fetch!(post);
decl_fetch!(put);
