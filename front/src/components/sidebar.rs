#[derive(Clone)]
pub enum Message {
    Error(String),
    Event(crate::Event),
    NeedUpdate,
    ReadAll,
    Update(oxfeed_common::Counts),
    UpdateAll(oxfeed_common::Counts),
}

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    pub current_route: super::app::Route,
}

#[derive(Clone)]
struct Links(Vec<Link>);

impl Links {
    fn new() -> Self {
        let links = vec![
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

        Self(links)
    }

    fn add_search(&mut self, search_route: &super::app::Route) {
        if !self.has_search() {
            self.0.push(Link {
                count: 0,
                icon: "search",
                label: "Search",
                route: search_route.clone(),
            });
        }
    }

    fn remove_search(&mut self) {
        if self.has_search() {
            self.0.pop();
        }
    }

    fn has_search(&self) -> bool {
        self.0.len() != Self::new().0.len()
    }

    fn update_count(&mut self, counts: &oxfeed_common::Counts) {
        self.0[0].count = counts.all;
        self.0[1].count = counts.unread;
        self.0[2].count = counts.favorites;
        self.0[3].count = counts.tags;
        self.0[4].count = counts.sources;
    }

    fn has_unread(&self) -> bool {
        self.0[1].count == 0
    }
}

impl std::ops::Deref for Links {
    type Target = Vec<Link>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Links {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone)]
struct Link {
    count: i64,
    icon: &'static str,
    label: &'static str,
    route: super::app::Route,
}

pub struct Component {
    current_route: super::app::Route,
    event_bus: yew_agent::Dispatcher<crate::event::Bus>,
    links: Links,
    _producer: Box<dyn yew_agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        use yew_agent::{Bridged, Dispatched};

        let callback = {
            let link = ctx.link().clone();
            move |e| link.send_message(Message::Event(e))
        };

        let component = Self {
            current_route: ctx.props().current_route.clone(),
            event_bus: crate::event::Bus::dispatcher(),
            links: Links::new(),
            _producer: crate::event::Bus::bridge(std::rc::Rc::new(callback)),
        };

        ctx.link().callback(|_| Message::NeedUpdate).emit(());

        component
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;

        match msg {
            Message::Event(event) => match event {
                crate::Event::ItemUpdate
                | crate::Event::SettingUpdate
                | crate::Event::SourceUpdate => ctx.link().send_message(Message::NeedUpdate),
                crate::Event::Redirected(route) => {
                    if route.starts_with("/search") {
                        self.links.add_search(&ctx.props().current_route);
                    } else {
                        self.links.remove_search();
                    }

                    should_render = true;
                }
                _ => (),
            },
            Message::Error(_) => (),
            Message::NeedUpdate => crate::api!(
                ctx.link(),
                counts() -> Message::Update
            ),
            Message::Update(counts) => {
                self.links.update_count(&counts);
                should_render = true;
            }
            Message::ReadAll => {
                ctx.link().send_future(async move {
                    if let Err(err) = crate::Api::items_read().await {
                        return Message::Error(err.to_string());
                    }

                    match crate::Api::counts().await {
                        Ok(count) => Message::UpdateAll(count),
                        Err(err) =>  Message::Error(err.to_string()),
                    }
                });
            }
            Message::UpdateAll(counts) => {
                self.event_bus.send(crate::event::Event::ItemUpdate);
                ctx.link().send_message(Message::Update(counts));
            }
        }

        should_render
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let favicon = if self.links.has_unread() {
            "/favicon.ico"
        } else {
            "/favicon-unread.ico"
        };

        if let Ok(Some(element)) = gloo::utils::document().query_selector("link[rel=icon]") {
            element.set_attribute("href", favicon).ok();
        }

        yew::html! {
            <>
                <button
                    class={ yew::classes!("btn", "btn-primary") }
                    onclick={ ctx.link().callback(|_| Message::ReadAll) }
                >{ "Mark all as read" }</button>
                <ul class="nav flex-column">
                {
                    for self.links.iter().map(move |link| yew::html! {
                        <li class="nav-item" data-bs-toggle="collapse" data-bs-target="#sidebarMenu">
                            <yew_router::components::Link<super::app::Route>
                                to={ link.route.clone() }
                                classes={ if link.route == self.current_route { "nav-link active" } else { "nav-link" } }
                            >
                                <super::Svg icon={ link.icon } size=16 />
                                { link.label }
                                {
                                    if link.count > 0 {
                                        yew::html! {
                                            <span
                                                class={ if link.route == self.current_route { "badge bg-primary" } else { "badge bg-secondary" } }
                                            >{ link.count }</span>
                                        }
                                    } else {
                                        "".into()
                                    }
                                }
                            </yew_router::components::Link<super::app::Route>>
                        </li>
                    })
                }
                </ul>
            </>
        }
    }

    crate::change!(current_route);
}
