#![recursion_limit="512"]

mod components;

use components::*;
use yew::services::console::ConsoleService as console;

#[derive(serde::Deserialize, Clone)]
struct Source {
    source_id: String,
    title: String,
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
    yew::initialize();
    yew::App::<Model>::new().mount_to_body();
}

pub(crate) fn fetch<T: yew::Component>(
    method: &str,
    link: &yew::ComponentLink<T>,
    url: &str,
) -> Result<yew::services::fetch::FetchTask, Box<dyn std::error::Error>>
where
    <T as yew::Component>::Message: std::convert::From<yew::format::Text>,
{
    let request = yew::services::fetch::Request::builder()
        .method(method)
        .uri(&format!("{}{}", env!("API_URL"), url))
        .body(yew::format::Nothing)?;

    let callback = link.callback(
        |response: yew::services::fetch::Response<yew::format::Text>| {
            <T as yew::Component>::Message::from(response.into_body())
        },
    );

    let fetch_task = yew::services::FetchService::fetch(request, callback)?;

    Ok(fetch_task)
}

macro_rules! decl_fetch {
    ($method:ident) => {
        pub(crate) fn $method<T: yew::Component>(
            link: &yew::ComponentLink<T>,
            url: &str,
        ) -> Result<yew::services::fetch::FetchTask, Box<dyn std::error::Error>>
        where
            <T as yew::Component>::Message: std::convert::From<yew::format::Text>,
        {
            fetch(stringify!($method), link, url)
        }

    }
}

decl_fetch!(get);
decl_fetch!(delete);
