#[derive(Clone)]
pub(crate) enum Message {
    Error(String),
    Event(crate::event::Message),
    NeedUpdate,
    Read,
    ReadAll,
    Update(crate::Counts),
}

impl std::convert::TryFrom<(http::Method, yew::format::Text)> for Message {
    type Error = serde_json::Error;

    fn try_from((method, response): (http::Method, yew::format::Text)) -> Result<Self, serde_json::Error> {
        let data = match response {
            Ok(data) => data,
            Err(err) => return Ok(Self::Error(err.to_string())),
        };

        let message = match method {
            http::Method::GET => Message::Update(serde_json::from_str(&data)?),
            http::Method::POST => Message::Read,
            _ => unreachable!(),
        };

        Ok(message)
    }
}

struct Link {
    count: usize,
    icon: &'static str,
    label: &'static str,
    url: &'static str,
}

pub(crate) struct Component {
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    fetch_task: Option<yew::services::fetch::FetchTask>,
    link: yew::ComponentLink<Self>,
    links: Vec<Link>,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::{Bridged, Dispatched};

        let callback = link.callback(|x| Self::Message::Event(x));

        let links = vec![
            Link {
                count: 0,
                icon: "collection",
                label: "All",
                url: "/",
            },
            Link {
                count: 0,
                icon: "book",
                label: "Unread",
                url: "/unread",
            },
            Link {
                count: 0,
                icon: "star",
                label: "Favorites",
                url: "/favorites",
            },
            Link {
                count: 0,
                icon: "diagram-3",
                label: "Sources",
                url: "/sources",
            },
            Link {
                count: 0,
                icon: "sliders",
                label: "Settings",
                url: "/settings",
            },
        ];

        let component = Self {
            event_bus: crate::event::Bus::dispatcher(),
            fetch_task: None,
            link,
            links,
            _producer: crate::event::Bus::bridge(callback),
        };

        component.link.callback(|_| Self::Message::NeedUpdate).emit(());

        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Error(error) => log::error!("{}", error),
            Self::Message::Event(_) => self.link.send_message(Self::Message::NeedUpdate),
            Self::Message::NeedUpdate => self.fetch_task = crate::get(&self.link, "/counts", yew::format::Nothing).ok(),
            Self::Message::Update(counts) => {
                self.links[0].count = counts.all;
                self.links[1].count = counts.unread;
                self.links[2].count = counts.favorites;
                self.links[3].count = counts.sources;
                self.fetch_task = None;
                return true;
            },
            Self::Message::Read => {
                self.event_bus.send(crate::event::Message::ItemUpdate);
            },
            Self::Message::ReadAll => self.fetch_task = crate::post(&self.link, "/items/read", yew::format::Nothing).ok(),
        }

        false
    }

    fn view(&self) -> yew::Html {
        let router = yew_router::service::RouteService::<()>::new();
        let current_url = router.get_path();

        yew::html! {
            <>
                <button
                    class=("btn", "btn-primary")
                    onclick=self.link.callback(|_| Self::Message::ReadAll)
                >{ "Mark all as read" }</button>
                <ul class="nav flex-column">
                {
                    for self.links.iter().map(|link| yew::html! {
                        <li class="nav-item">
                            <a
                                href={ link.url }
                                class=if link.url == current_url { "nav-link active" } else { "nav-link" }
                            >
                                <super::Svg icon=link.icon size=16 />
                                { link.label }
                                {
                                    if link.count > 0 {
                                        yew::html! {
                                            <span class="badge badge-primary">{ link.count }</span>
                                        }
                                    } else {
                                        "".into()
                                    }
                                }
                            </a>
                        </li>
                    })
                }
                </ul>
            </>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
