#[derive(Clone, Debug, PartialEq, Eq, yew_router::Routable)]
pub(crate) enum Route {
    #[at("/favorites")]
    Favorites,
    #[at("/settings")]
    Settings,
    #[at("/search/:kind")]
    Search { kind: String },
    #[at("/sources")]
    Sources,
    #[at("/tags")]
    Tags,
    #[at("/unread")]
    Unread,
    #[at("/all")]
    All,
    #[at("/")]
    Index,
    #[not_found]
    #[at("/not-found")]
    NotFound,
}

#[yew::function_component]
pub(crate) fn Component() -> yew::Html {
    use wasm_bindgen::JsCast as _;

    let context = yew::use_reducer(crate::Context::default);
    let auth = yew::use_memo(context.clone(), |context| context.auth);

    let _ = yew::use_state(|| websocket(context.clone()));

    let on_visibility_change = {
        let context = context.clone();

        yew::use_state(move || {
            wasm_bindgen::closure::Closure::<dyn Fn()>::wrap(Box::new(move || {
                let document = gloo::utils::document();

                if document.visibility_state() == web_sys::VisibilityState::Visible {
                    context.dispatch(crate::Action::NeedUpdate);
                }
            }))
        })
    };

    let document = gloo::utils::document();
    document.set_onvisibilitychange((*on_visibility_change).as_ref().dyn_ref());

    yew::html! {
        <yew::ContextProvider<yew::UseReducerHandle<crate::Context>> {context}>
            if *auth {
                <yew_router::router::BrowserRouter>
                    <yew_router::Switch<Route> render={ switch } />
                </yew_router::router::BrowserRouter>
            } else {
                <super::Login />
            }
        </yew::ContextProvider<yew::UseReducerHandle<crate::Context>>>
    }
}

fn websocket(context: yew::UseReducerHandle<crate::Context>) -> Option<wasm_sockets::EventClient> {
    let url = env!("API_URL")
        .replace("http://", "ws://")
        .replace("https://", "wss://");

    let ws_url = format!("{url}/ws?token={}", crate::Api::token());

    match wasm_sockets::EventClient::new(&ws_url) {
        Ok(mut websocket) => {
            {
                let context = context.clone();
                websocket.set_on_message(Some(Box::new(move |_, _| {
                    context.dispatch(crate::Action::NeedUpdate);
                })));
            }
            {
                let context = context.clone();
                websocket.set_on_error(Some(Box::new(move |error| {
                    log::error!("{error:?}");
                    context.dispatch(crate::Action::WebsocketError);
                })));
            }
            {
                let context = context.clone();
                websocket.set_on_close(Some(Box::new(move |event| {
                    log::error!("{event:?}");
                    context.dispatch(crate::Action::WebsocketError);
                })));
            }

            Some(websocket)
        }
        Err(err) => {
            context.dispatch(crate::Action::WebsocketError);
            log::error!("Unable to connect to websocket: {err}");
            None
        }
    }
}

fn switch(route: Route) -> yew::Html {
    yew::html! {
        <>
            <nav class="navbar navbar-dark sticky-top bg-dark flex-md-nowrap p-0 shadow">
                <super::Header />
            </nav>
            <div class="container-fluid">
                <div class="row">
                    <nav id="sidebarMenu" class="col-md-3 col-lg-2 d-md-block bg-light sidebar collapse">
                        <super::Sidebar current_route={ route.clone() } />
                    </nav>
                    <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
                        <super::Alerts />
                        {
                            match route {
                                Route::All => yew::html!{<super::Items kind="all" />},
                                Route::Favorites => yew::html!{<super::Items kind="favorites" />},
                                Route::Settings => yew::html!{<super::Settings />},
                                Route::Sources => yew::html!{<super::Sources />},
                                Route::Tags => yew::html!{<super::Tags />},
                                Route::Unread => yew::html!{<super::Items kind="unread" />},
                                Route::Search { kind } => yew::html!{<super::Search kind={ kind.clone() } />},
                                Route::NotFound => yew::html!{<super::NotFound />},
                                Route::Index => yew::html!{<yew_router::prelude::Redirect<Route> to={ Route::Unread } />},
                            }
                        }
                    </main>
                </div>
            </div>
        </>
    }
}
