#[derive(Clone)]
pub(crate) enum Message {
    Event(crate::event::Event),
    NeedUpdate,
    Read,
    ReadAll,
    Update(oxfeed_common::Counts),
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::Counts(counts) => Self::Update(counts),
            crate::event::Api::ItemsRead => Self::Read,
            _ => unreachable!(),
        }
    }
}

struct Link {
    count: i64,
    icon: &'static str,
    label: &'static str,
    url: String,
}

pub(crate) struct Component {
    api: crate::Api<Self>,
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    link: yew::ComponentLink<Self>,
    links: Vec<Link>,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::{Bridged, Dispatched};

        let callback = link.callback(Self::Message::Event);

        let mut links = vec![
            Link {
                count: 0,
                icon: "collection",
                label: "All",
                url: "/".to_string(),
            },
            Link {
                count: 0,
                icon: "book",
                label: "Unread",
                url: "/unread".to_string(),
            },
            Link {
                count: 0,
                icon: "star",
                label: "Favorites",
                url: "/favorites".to_string(),
            },
            Link {
                count: 0,
                icon: "diagram-3",
                label: "Sources",
                url: "/sources".to_string(),
            },
            Link {
                count: 0,
                icon: "sliders",
                label: "Settings",
                url: "/settings".to_string(),
            },
        ];

        let route = yew_router::service::RouteService::<()>::new().get_path();

        if route.starts_with("/search") {
            links.push(Link {
                count: 0,
                icon: "search",
                label: "Search",
                url: route,
            });
        }

        let component = Self {
            api: crate::Api::new(link.clone()),
            event_bus: crate::event::Bus::dispatcher(),
            link,
            links,
            _producer: crate::event::Bus::bridge(callback),
        };

        component
            .link
            .callback(|_| Self::Message::NeedUpdate)
            .emit(());

        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Event(event) => match event {
                crate::event::Event::ItemUpdate
                | crate::event::Event::SettingUpdate
                | crate::event::Event::SourceUpdate => {
                    self.link.send_message(Self::Message::NeedUpdate)
                }
                _ => (),
            },
            Self::Message::NeedUpdate => self.api.counts(),
            Self::Message::Update(counts) => {
                self.links[0].count = counts.all;
                self.links[1].count = counts.unread;
                self.links[2].count = counts.favorites;
                self.links[3].count = counts.sources;

                return true;
            }
            Self::Message::Read => self.event_bus.send(crate::event::Event::ItemUpdate),
            Self::Message::ReadAll => self.api.items_read(),
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
                                href={ link.url.as_str() }
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
