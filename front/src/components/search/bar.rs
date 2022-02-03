pub(crate) enum Message {
    Input(String),
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub current_route: crate::components::app::Route,
    #[prop_or_default]
    pub filter: crate::Filter,
}

pub(crate) struct Component {
    current_route: crate::components::app::Route,
    event_bus: yew_agent::Dispatcher<crate::event::Bus>,
    filter: crate::Filter,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        use yew_agent::Dispatched;

        Self {
            current_route: ctx.props().current_route.clone(),
            filter: crate::Filter::new(),
            event_bus: crate::event::Bus::dispatcher(),
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        let Message::Input(value) = msg;

        self.filter = value.into();

        let location = crate::Location::new();
        let mut route = location.path();

        if route.starts_with("/search") {
            route = route.trim_start_matches("/search").to_string();
        }

        if !self.filter.is_empty() {
            route = format!("/search{route}?{}", self.filter.to_url_param());
        }

        self.event_bus.send(crate::Event::Redirect(route));

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        if matches!(self.current_route, crate::components::app::Route::Settings) {
            "".into()
        } else {
            yew::html! {
                <input
                    class={ yew::classes!("form-control", "form-control-dark") }
                    type="text"
                    value={ self.filter.to_string() }
                    placeholder="Search"
                    aria-label="Search"
                    oninput={ ctx.link().callback(|e: yew::InputEvent| {
                        use yew::TargetCast;

                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        Message::Input(input.value())
                    }) }
                />
            }
        }
    }

    crate::change!(current_route, filter);
}
