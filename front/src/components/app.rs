#[derive(Clone, yew_router::Switch, Debug)]
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
    Route(yew_router::route::Route<()>),
    Websocket(WebsocketAction),
}

impl From<crate::event::Api> for Message {
    fn from(_: crate::event::Api) -> Self {
        unreachable!()
    }
}

#[derive(Debug)]
pub(crate) enum WebsocketAction {
    Ready(Result<String, anyhow::Error>),
    Status(yew::services::websocket::WebSocketStatus),
}

pub(crate) struct Component {
    auth: bool,
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    location: crate::Location,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
    _websocket: yew::services::websocket::WebSocketTask,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::{Bridged, Dispatched};

        let event_cb = link.callback(Self::Message::Event);

        let url = env!("API_URL")
            .replace("http://", "ws://")
            .replace("https://", "wss://");

        let ws_url = format!("{}/ws?token={}", url, crate::Api::<Self>::token());
        let ws_cb = link.callback(|data| Self::Message::Websocket(WebsocketAction::Ready(data)));
        let ws_notif =
            link.callback(|status| Self::Message::Websocket(WebsocketAction::Status(status)));

        let url_cb = link.callback(Self::Message::Route);
        let mut location = crate::Location::new();
        location.register_callback(url_cb);

        Self {
            auth: true,
            event_bus: crate::event::Bus::dispatcher(),
            location,
            _producer: crate::event::Bus::bridge(event_cb),
            _websocket: yew::services::websocket::WebSocketService::connect_text(
                &ws_url, ws_cb, ws_notif,
            )
            .unwrap(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Event(event) => match event {
                crate::event::Event::Api(crate::event::Api::Auth) => self.auth = true,
                crate::event::Event::AuthRequire => self.auth = false,
                crate::event::Event::Redirect(route) => self.location.set_path(&route),
                _ => return false,
            },
            Self::Message::Route(_) => (),
            Self::Message::Websocket(event) => match event {
                WebsocketAction::Ready(_) => {
                    self.event_bus.send(crate::event::Event::ItemUpdate);
                    return false;
                }
                WebsocketAction::Status(_) => return false,
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

        use yew_router::Switch;
        let route_service = yew_router::service::RouteService::<()>::new();
        let route = Route::switch(route_service.get_route()).unwrap_or(Route::All);

        let pagination: oxfeed_common::Pagination = (&self.location).into();

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
                            {
                                match route {
                                    Route::All => yew::html!{<super::Items kind="all" pagination=pagination />},
                                    Route::Favorites => yew::html!{<super::Items kind="favorites" pagination=pagination />},
                                    Route::Settings => yew::html!{<super::Settings />},
                                    Route::Sources => yew::html!{<super::Sources pagination=pagination />},
                                    Route::Unread => yew::html!{<super::Items kind="unread" pagination=pagination />},
                                    Route::Search(kind) => yew::html!{<super::Search kind=kind pagination=pagination />},
                                }
                            }
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
