#[derive(yew_router::Switch, Clone)]
enum Route {
    #[to = "/favorites"]
    Favorites,
    #[to = "/settings"]
    Settings,
    #[to = "/search/{}"]
    Search(String),
    #[to = "/sources"]
    Sources,
    #[to = "/unread"]
    Unread,
    #[to = "/"]
    All,
}

pub(crate) struct Component {
    pagination: crate::Pagination,
}

impl yew::Component for Component {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self {
            pagination: crate::Location::new().into(),
        }
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        use yew_router::router::Router;

        let pagination = self.pagination;

        yew::html! {
            <>
                <nav class="navbar navbar-dark sticky-top bg-dark flex-md-nowrap p-0 shadow">
                    <super::Header />
                </nav>
                <div class="container-fluid">
                    <div class="row">
                        <nav id="sidebarMenu" class="col-md-3 col-lg-2 d-md-block bg-light sidebar collapse">
                            <super::Sidebar />
                        </nav>
                        <main class="col-md-9 ml-sm-auto col-lg-10">
                            <super::Alerts />
                            <Router<Route, ()>
                                render = yew_router::router::Router::render(move |switch: Route| {
                                    match switch {
                                        Route::All => yew::html!{<super::Items kind="all" pagination=pagination />},
                                        Route::Favorites => yew::html!{<super::Items kind="favorites" pagination=pagination />},
                                        Route::Settings => yew::html!{<super::Settings />},
                                        Route::Sources => yew::html!{<super::Sources pagination=pagination />},
                                        Route::Unread => yew::html!{<super::Items kind="unread" pagination=pagination />},
                                        Route::Search(kind) => yew::html!{<super::Search kind=kind pagination=pagination />},
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
