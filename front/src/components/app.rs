#[derive(Clone, PartialEq, Eq, yew_router::Routable)]
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

pub(crate) enum Message {
    Event(crate::Event),
    Websocket(wasm_sockets::Message),
}

pub(crate) struct Component {
    auth: bool,
    event_bus: yew_agent::Dispatcher<crate::event::Bus>,
    _producer: Box<dyn yew_agent::Bridge<crate::event::Bus>>,
    _websocket: Option<wasm_sockets::EventClient>,
}

impl Component {
    fn websocket(link: &yew::html::Scope<Self>) -> Option<wasm_sockets::EventClient> {
        let url = env!("API_URL")
            .replace("http://", "ws://")
            .replace("https://", "wss://");

        let ws_url = format!("{}/ws?token={}", url, crate::Api::token());

        match wasm_sockets::EventClient::new(&ws_url) {
            Ok(mut websocket) => {
                let link = link.clone();

                websocket.set_on_message(Some(Box::new(move |_, msg| {
                    link.send_message(Message::Websocket(msg));
                })));

                Some(websocket)
            }
            Err(err) => {
                log::error!("Unable to connect to websocket: {}", err);
                None
            }
        }
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        use yew_agent::{Bridged, Dispatched};

        let event_cb = ctx.link().callback(Message::Event);

        Self {
            _websocket: Self::websocket(ctx.link()),
            auth: !crate::Api::token().is_empty(),
            event_bus: crate::event::Bus::dispatcher(),
            _producer: crate::event::Bus::bridge(event_cb),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;

        match msg {
            Message::Event(event) => match event {
                crate::Event::AuthRequire => {
                    self.auth = false;
                    should_render = true;
                }
                crate::Event::Logged => {
                    self.auth = true;
                    self._websocket = Self::websocket(ctx.link());
                    should_render = true;
                }
                crate::Event::Redirect(route) => {
                    crate::location::set_route(&route);
                    should_render = true;
                }
                _ => (),
            },
            Message::Websocket(_) => self.event_bus.send(crate::Event::ItemUpdate),
        }

        should_render
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        if !self.auth {
            return yew::html! {
                <super::Login />
            };
        }

        let pagination: oxfeed_common::Pagination = crate::Location::new().into();

        yew::html! {
            <yew_router::router::BrowserRouter>
                <yew_router::Switch<Route>
                    render={ yew_router::Switch::render(move |route: &Route| {
                        yew::html! {
                            <>
                                <nav class="navbar navbar-dark sticky-top bg-dark flex-md-nowrap p-0 shadow">
                                    <super::Header current_route={ route.clone() } />
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
                                                    Route::All => yew::html!{<super::Items kind="all" pagination={ pagination } />},
                                                    Route::Favorites => yew::html!{<super::Items kind="favorites" pagination={ pagination } />},
                                                    Route::Settings => yew::html!{<super::Settings />},
                                                    Route::Sources => yew::html!{<super::Sources pagination={ pagination } />},
                                                    Route::Tags => yew::html!{<super::Tags pagination={ pagination } />},
                                                    Route::Unread => yew::html!{<super::Items kind="unread" pagination={ pagination } />},
                                                    Route::Search { kind } => yew::html!{<super::Search kind={ kind.clone() } pagination={ pagination } />},
                                                    Route::NotFound => yew::html!{<super::NotFound />},
                                                    Route::Index => yew::html!{<yew_router::prelude::Redirect<Route> to={ Route::Unread } />},
                                                }
                                            }
                                        </main>
                                    </div>
                                </div>
                            </>
                        }
                    }) }
                />
            </yew_router::router::BrowserRouter>
        }
    }

    crate::change!();
}
