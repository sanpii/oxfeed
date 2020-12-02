pub(crate) enum Message {
    Logout,
    Loggedout,
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::Auth => Self::Loggedout,
            _ => unreachable!(),
        }
    }
}

pub(crate) struct Component {
    api: crate::Api<Self>,
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    link: yew::ComponentLink<Self>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Dispatched;

        Self {
            api: crate::Api::new(link.clone()),
            event_bus: crate::event::Bus::dispatcher(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Logout => self.api.auth_logout(),
            Self::Message::Loggedout => {
                let alert = crate::event::Alert::info("Logged out");
                self.event_bus.send(crate::event::Event::Alert(alert));
                crate::Location::new().reload();
            },
        }

        false
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <>
                <a class="navbar-brand col-md-3 col-lg-2 mr-0 px-3" href="#">{{ "Oxfeed" }}</a>
                <button class="navbar-toggler position-absolute d-md-none collapsed" type="button" data-toggle="collapse" data-target="#sidebarMenu" aria-controls="sidebarMenu" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <super::search::Bar />
                <button
                    class=("btn", "btn-secondary")
                    title="Logout"
                    onclick=self.link.callback(|_| Self::Message::Logout)
                >
                    <super::Svg icon="door-closed" size=24 />
                </button>
            </>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
