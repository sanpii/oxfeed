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

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub current_route: super::app::Route,
}

#[derive(Clone)]
struct Link {
    count: i64,
    icon: &'static str,
    label: &'static str,
    route: super::app::Route,
}

pub(crate) struct Component {
    api: crate::Api<Self>,
    current_route: super::app::Route,
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    link: yew::ComponentLink<Self>,
    links: Vec<Link>,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::{Bridged, Dispatched};

        let callback = link.callback(Self::Message::Event);

        let mut links = vec![
            Link {
                count: 0,
                icon: "collection",
                label: "All",
                route: super::app::Route::All,
            },
            Link {
                count: 0,
                icon: "book",
                label: "Unread",
                route: super::app::Route::Unread,
            },
            Link {
                count: 0,
                icon: "star",
                label: "Favorites",
                route: super::app::Route::Favorites,
            },
            Link {
                count: 0,
                icon: "tags",
                label: "Tags",
                route: super::app::Route::Tags,
            },
            Link {
                count: 0,
                icon: "diagram-3",
                label: "Sources",
                route: super::app::Route::Sources,
            },
            Link {
                count: 0,
                icon: "sliders",
                label: "Settings",
                route: super::app::Route::Settings,
            },
        ];

        if let super::app::Route::Search(_) = props.current_route {
            links.push(Link {
                count: 0,
                icon: "search",
                label: "Search",
                route: props.current_route.clone(),
            });
        }

        let component = Self {
            api: crate::Api::new(link.clone()),
            current_route: props.current_route,
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
                self.links[3].count = counts.tags;
                self.links[4].count = counts.sources;

                return true;
            }
            Self::Message::Read => self.event_bus.send(crate::event::Event::ItemUpdate),
            Self::Message::ReadAll => self.api.items_read(),
        }

        false
    }

    fn view(&self) -> yew::Html {
        let favicon = if self.links[1].count == 0 {
            "/favicon.ico"
        } else {
            "/favicon-unread.ico"
        };

        if let Ok(Some(element)) = yew::utils::document().query_selector("link[rel=icon]") {
            element.set_attribute("href", &favicon).ok();
        }

        yew::html! {
            <>
                <button
                    class=yew::classes!("btn", "btn-primary")
                    onclick=self.link.callback(|_| Self::Message::ReadAll)
                >{ "Mark all as read" }</button>
                <ul class="nav flex-column">
                {
                    for self.links.iter().map(move |link| yew::html! {
                        <li class="nav-item">
                            <yew_router::components::RouterAnchor<super::app::Route>
                                route=link.route.clone()
                                classes=if link.route == self.current_route { "nav-link active" } else { "nav-link" }
                            >
                                <super::Svg icon=link.icon size=16 />
                                { link.label }
                                {
                                    if link.count > 0 {
                                        yew::html! {
                                            <span
                                                class=if link.route == self.current_route { "badge bg-primary" } else { "badge bg-secondary" }
                                            >{ link.count }</span>
                                        }
                                    } else {
                                        "".into()
                                    }
                                }
                            </yew_router::components::RouterAnchor<super::app::Route>>
                        </li>
                    })
                }
                </ul>
            </>
        }
    }

    crate::change!(current_route);
}
