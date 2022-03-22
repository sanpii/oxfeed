pub(crate) enum Message {
    Error(String),
    Logout,
    Loggedout,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub current_route: super::app::Route,
}

pub(crate) struct Component {
    current_route: super::app::Route,
    event_bus: yew_agent::Dispatcher<crate::event::Bus>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        use yew_agent::Dispatched;

        Self {
            current_route: ctx.props().current_route.clone(),
            event_bus: crate::event::Bus::dispatcher(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Error(_) => (),
            Message::Logout => crate::api!(
                ctx.link(),
                auth_logout() -> |_| Message::Loggedout
            ),
            Message::Loggedout => {
                let alert = crate::event::Alert::info("Logged out");
                self.event_bus.send(crate::Event::Alert(alert));
                self.event_bus.send(crate::Event::Redirect("/".to_string()));
            }
        }

        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let filter = crate::Filter::new();

        yew::html! {
            <>
                <a class="navbar-brand col-md-3 col-lg-2 me-0 px-3" href="#">{{ "Oxfeed" }}</a>
                <button class="navbar-toggler position-absolute d-md-none collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#sidebarMenu" aria-controls="sidebarMenu" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <super::search::Bar current_route={ self.current_route.clone() } filter={ filter } />
                <button
                    class={ yew::classes!("btn", "btn-secondary", "logout") }
                    title="Logout"
                    onclick={ ctx.link().callback(|_| Message::Logout) }
                >
                    <super::Svg icon="door-closed" size=24 />
                </button>
            </>
        }
    }

    crate::change!(current_route);
}
