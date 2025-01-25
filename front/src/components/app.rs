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
    let theme = yew::use_memo(context.clone(), |context| context.theme);
    let _sse = yew::use_state(|| sse(context.clone()));

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

    yew::use_effect_with((), |_| {
        change_rss();
    });

    yew::use_effect_with(theme, |theme| {
        let document = gloo::utils::document();
        let Some(root) = document.document_element() else {
            return;
        };

        root.set_attribute("data-bs-theme", &theme.to_string().to_lowercase())
            .ok();
    });

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

fn sse(
    context: yew::UseReducerHandle<crate::Context>,
) -> gloo::net::eventsource::futures::EventSource {
    use futures::StreamExt as _;

    let url = format!("{}/sse?token={}", env!("API_URL"), crate::Api::token());
    let mut es = gloo::net::eventsource::futures::EventSource::new(&url).unwrap();
    let mut stream = es.subscribe("message").unwrap();

    yew::platform::spawn_local(async move {
        while let Some(Ok(_)) = stream.next().await {
            context.dispatch(crate::Action::NeedUpdate);
        }
        context.dispatch(crate::Action::SseError);
    });

    es
}

fn change_rss() {
    yew::platform::spawn_local(async move {
        let Ok(me) = crate::Api::auth().await else {
            return;
        };

        let document = gloo::utils::document();

        if let Ok(Some(element)) = document.query_selector("link[type=\"application/rss+xml\"]") {
            let href = format!("{}/rss/{}", env!("API_URL"), me.id);
            element.set_attribute("href", &href).ok();
        }
    });
}

fn switch(route: Route) -> yew::Html {
    yew::html! {
        <>
            <nav class="navbar navbar-dark navbar-expand-lg sticky-top bg-dark flex-md-nowrap p-0 shadow">
                <super::Header />
            </nav>
            <div class="container-fluid">
                <div class="row">
                    <nav id="sidebarMenu" class="col-md-3 col-lg-2 d-md-block sidebar collapse">
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
