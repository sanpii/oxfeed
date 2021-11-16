pub(crate) enum Message {
    Error(oxfeed_common::Error),
    Logout,
    Loggedout,
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub current_route: super::app::Route,
}

pub(crate) struct Component {
    current_route: super::app::Route,
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    link: yew::ComponentLink<Self>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Dispatched;

        Self {
            current_route: props.current_route,
            event_bus: crate::event::Bus::dispatcher(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Error(err) => self.event_bus.send(err.into()),
            Self::Message::Logout => {
                crate::api!(
                    self.link,
                    auth_logout() -> |_| Self::Message::Loggedout, Self::Message::Error
                );
            }
            Self::Message::Loggedout => {
                let alert = crate::event::Alert::info("Logged out");
                self.event_bus.send(crate::event::Event::Alert(alert));
                crate::Location::new().reload();
            }
        }

        false
    }

    fn view(&self) -> yew::Html {
        let filter = crate::Filter::new();

        yew::html! {
            <>
                <a class="navbar-brand col-md-3 col-lg-2 me-0 px-3" href="#">{{ "Oxfeed" }}</a>
                <button class="navbar-toggler position-absolute d-md-none collapsed" type="button" data-toggle="collapse" data-target="#sidebarMenu" aria-controls="sidebarMenu" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <super::search::Bar current_route=self.current_route.clone() filter=filter />
                <button
                    class=yew::classes!("btn", "btn-secondary", "logout")
                    title="Logout"
                    onclick=self.link.callback(|_| Self::Message::Logout)
                >
                    <super::Svg icon="door-closed" size=24 />
                </button>
            </>
        }
    }

    crate::change!(current_route);
}
