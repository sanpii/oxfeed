pub(crate) enum Message {
    Input(String),
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub current_route: crate::components::app::Route,
    #[prop_or_default]
    pub filter: crate::Filter,
}

pub(crate) struct Component {
    current_route: crate::components::app::Route,
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    filter: crate::Filter,
    link: yew::ComponentLink<Self>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Dispatched;

        Self {
            current_route: props.current_route,
            filter: crate::Filter::new(),
            event_bus: crate::event::Bus::dispatcher(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Message::Input(value) => {
                self.filter = value.into();

                let location = crate::Location::new();
                let mut route = location.path();

                if route.starts_with("/search") {
                    route = route.trim_start_matches("/search").to_string();
                }

                route = format!("/search{}?{}", route, self.filter.to_url_param());

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
                    class=yew::classes!("form-control", "form-control-dark")
                    type="text"
                    value=self.filter.to_string()
                    placeholder="Search"
                    aria-label="Search"
                    oninput=self.link.callback(|e: yew::InputData| Message::Input(e.value))
                />
            }
        }
    }

    crate::change!(current_route, filter);
}
