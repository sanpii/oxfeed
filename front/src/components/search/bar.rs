pub(crate) enum Message {
    Input(String),
}

pub(crate) struct Component {
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    link: yew::ComponentLink<Self>,
    route: String,
    query: String,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Dispatched;

        let location = crate::Location::new();
        let path = match location.path().as_str() {
            "/" => "/all".to_string(),
            path => path.to_string(),
        };

        let route = if path.starts_with("/search") {
            path
        } else {
            format!("/search{}", path)
        };

        Self {
            event_bus: crate::event::Bus::dispatcher(),
            link,
            route,
            query: location.query().get("q").cloned().unwrap_or_default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Input(value) => {
                self.query = value;

                let route = if self.query.is_empty() {
                    self.route.trim_start_matches("/search").to_string()
                } else {
                    format!("{}?q={}", self.route, self.query)
                };

                self.event_bus.send(crate::event::Event::Redirect(route));
            }
        }

        true
    }

    fn view(&self) -> yew::Html {
        if self.route == "/settings" {
            "".into()
        } else {
            yew::html! {
                <input
                    class=("form-control", "form-control-dark")
                    type="text"
                    value=self.query
                    placeholder="Search"
                    aria-label="Search"
                    oninput=self.link.callback(|e: yew::InputData| Self::Message::Input(e.value))
                />
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
