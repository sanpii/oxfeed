#[derive(Clone, PartialEq, Eq, yew_router::Switch)]
pub(crate) enum Route {
    #[to = "/favorites"]
    Favorites,
    #[to = "/settings"]
    Settings,
    #[to = "/search/{}"]
    Search(String),
    #[to = "/sources"]
    Sources,
    #[to = "/tags"]
    Tags,
    #[to = "/unread"]
    Unread,
    #[to = "/all"]
    All,
    #[to = "/!"]
    Index,
    NotFound,
}

pub(crate) enum Message {
    Event(crate::Event),
    Index,
    Websocket(WebsocketAction),
}

pub(crate) enum WebsocketAction {
    Ready(Result<String, anyhow::Error>),
    Status(yew::services::websocket::WebSocketStatus),
}

pub(crate) struct Component {
    auth: bool,
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    link: yew::ComponentLink<Self>,
    location: crate::Location,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
    _websocket: Option<yew::services::websocket::WebSocketTask>,
}

impl Component {
    fn websocket(
        link: &yew::ComponentLink<Self>,
    ) -> Option<yew::services::websocket::WebSocketTask> {
        let url = env!("API_URL")
            .replace("http://", "ws://")
            .replace("https://", "wss://");

        let ws_url = format!("{}/ws?token={}", url, crate::Api::token());
        let ws_cb = link.callback(|data| Message::Websocket(WebsocketAction::Ready(data)));
        let ws_notif = link.callback(|status| Message::Websocket(WebsocketAction::Status(status)));

        match yew::services::websocket::WebSocketService::connect_text(&ws_url, ws_cb, ws_notif) {
            Ok(websocket) => Some(websocket),
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

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::{Bridged, Dispatched};

        let event_cb = link.callback(Message::Event);

        Self {
            _websocket: Self::websocket(&link),
            auth: !crate::Api::token().is_empty(),
            event_bus: crate::event::Bus::dispatcher(),
            link,
            location: crate::Location::new(),
            _producer: crate::event::Bus::bridge(event_cb),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        let mut should_render = false;

        match msg {
            Message::Event(event) => match event {
                crate::Event::AuthRequire => {
                    self.auth = false;
                    should_render = true;
                }
                crate::Event::Logged => {
                    self.auth = true;
                    self._websocket = Self::websocket(&self.link);
                    should_render = true;
                }
                crate::Event::Redirect(route) => {
                    self.location.set_path(&route);
                    should_render = true;
                }
                _ => (),
            },
            Message::Index =>  self.event_bus.send(crate::Event::Redirect("/unread".to_string())),
            Message::Websocket(event) => match event {
                WebsocketAction::Ready(_) => self.event_bus.send(crate::Event::ItemUpdate),
                WebsocketAction::Status(_) => (),
            },
        }

        should_render
    }

    fn view(&self) -> yew::Html {
        if !self.auth {
            return yew::html! {
                <super::Login />
            };
        }

        if self.location.path() == "/" {
            self.link.send_message(Message::Index);
            return "Redirecting...".into();
        }

        let pagination: oxfeed_common::Pagination = (&self.location).into();

        yew::html! {
            <yew_router::router::Router<Route>
                redirect=yew_router::router::Router::redirect(|_| Route::NotFound)
                render=yew_router::router::Router::render(move |route: Route| {
                    yew::html! {
                        <>
                            <nav class="navbar navbar-dark sticky-top bg-dark flex-md-nowrap p-0 shadow">
                                <super::Header current_route=route.clone() />
                            </nav>
                            <div class="container-fluid">
                                <div class="row">
                                    <nav id="sidebarMenu" class="col-md-3 col-lg-2 d-md-block bg-light sidebar collapse">
                                        <super::Sidebar current_route=route.clone() />
                                    </nav>
                                    <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
                                        <super::Alerts />
                                        {
                                            match route {
                                                Route::All => yew::html!{<super::Items kind="all" pagination=pagination />},
                                                Route::Favorites => yew::html!{<super::Items kind="favorites" pagination=pagination />},
                                                Route::Settings => yew::html!{<super::Settings />},
                                                Route::Sources => yew::html!{<super::Sources pagination=pagination />},
                                                Route::Tags => yew::html!{<super::Tags pagination=pagination />},
                                                Route::Unread => yew::html!{<super::Items kind="unread" pagination=pagination />},
                                                Route::Search(kind) => yew::html!{<super::Search kind=kind pagination=pagination />},
                                                Route::NotFound => yew::html!{<super::NotFound />},
                                                Route::Index => unreachable!(),
                                            }
                                        }
                                    </main>
                                </div>
                            </div>
                        </>
                    }
                })
            />
        }
    }

    crate::change!();
}
