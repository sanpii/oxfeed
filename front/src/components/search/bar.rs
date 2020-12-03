pub(crate) enum Message {
    Input(String),
    Nope,
    Search,
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
                self.query = value.clone();

                if crate::Location::new().path().starts_with("/search") {
                    self.event_bus.send(crate::event::Event::Search(value));
                }
            }
            Self::Message::Nope => return false,
            Self::Message::Search => {
                let location = crate::Location::new();
                location.redirect(&format!("{}?q={}", self.route, self.query));
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
                    onkeydown=self.link.callback(|e: yew::KeyboardEvent| match e.key().as_str() {
                        "Enter" => Self::Message::Search,
                        _ => Self::Message::Nope,
                    })
                />
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
