#[derive(yew_router::Switch, Clone)]
enum Route {
    #[to = "/favorites"]
    Favorites,
    #[to = "/settings"]
    Settings,
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
                    <super::Header />
                </nav>
                <div class="container-fluid">
                    <div class="row">
                        <nav id="sidebarMenu" class="col-md-3 col-lg-2 d-md-block bg-light sidebar collapse">
                            <super::Sidebar />
                        </nav>
                        <main class="col-md-9 ml-sm-auto col-lg-10">
                            <Router<Route, ()>
                                render = yew_router::router::Router::render(move |switch: Route| {
                                    match switch {
                                        Route::All => yew::html!{<super::All pagination=pagination />},
                                        Route::Favorites => yew::html!{<super::Favorites pagination=pagination />},
                                        Route::Settings => yew::html!{<super::Settings />},
                                        Route::Sources => yew::html!{<super::Sources pagination=pagination />},
                                        Route::Unread => yew::html!{<super::Unread pagination=pagination />},
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
