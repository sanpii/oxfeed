pub(crate) enum Message {
    Input(String),
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub current_route: crate::components::app::Route,
    #[prop_or_default]
    query: String,
}

pub(crate) struct Component {
    current_route: crate::components::app::Route,
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    query: String,
    link: yew::ComponentLink<Self>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Dispatched;

        let location = crate::Location::new();

        Self {
            current_route: props.current_route,
            query: location.q(),
            event_bus: crate::event::Bus::dispatcher(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Input(value) => {
                let location = crate::Location::new();
                let mut route = match location.path().as_str() {
                    "/" => "/all".to_string(),
                    route => route.to_string(),
                };

                if route.starts_with("/search") {
                    route = route.trim_start_matches("/search").to_string();
                }

                if !value.is_empty() {
                    route = format!("/search{}?q={}", route, value);
                }

                self.query = value;
                self.event_bus.send(crate::event::Event::Redirect(route));
            }
        }

        true
    }

    fn view(&self) -> yew::Html {
        if matches!(self.current_route, crate::components::app::Route::Settings) {
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

    crate::change!(current_route, query);
}
