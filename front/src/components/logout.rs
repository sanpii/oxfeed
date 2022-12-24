pub enum Message {
    Error(String),
    Logout,
    Loggedout,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    pub button: bool,
}

pub struct Component {
    button: bool,
    event_bus: yew_agent::Dispatcher<crate::event::Bus>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        use yew_agent::Dispatched;

        Self {
            button: ctx.props().button,
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
        if self.button {
            yew::html! {
                    <button
                        class={ yew::classes!("btn", "btn-secondary", "logout", "d-none", "d-md-block") }
                        title="Logout"
                        onclick={ ctx.link().callback(|_| Message::Logout) }
                    >
                        <super::Svg icon="door-closed" size=24 />
                    </button>
            }
        } else {
            yew::html! {
                <a class="nav-link" onclick={ ctx.link().callback(|_| Message::Logout) } href="#">
                    <super::Svg icon="door-closed" size=24 /> { "Logout" }
                </a>
            }
        }
    }

    crate::change!(button);
}
