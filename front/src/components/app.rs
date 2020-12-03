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

pub(crate) enum Message {
    Event(crate::event::Event),
}

pub(crate) struct Component {
    auth: bool,
    pagination: crate::Pagination,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Bridged;

        let callback = link.callback(|x| Self::Message::Event(x));

        Self {
            auth: true,
            pagination: crate::Location::new().into(),
            _producer: crate::event::Bus::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Event(event) => match event {
                crate::event::Event::Api(crate::event::Api::Auth) => self.auth = true,
                crate::event::Event::AuthRequire => self.auth = false,
                _ => return false,
            },
        }

        true
    }

    fn view(&self) -> yew::Html {
        if !self.auth {
            return yew::html! {
                <super::Login />
            };
        }

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
