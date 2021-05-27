pub(crate) enum Message {
    Toggle,
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub id: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub on_toggle: yew::Callback<bool>,
}

pub(crate) struct Component {
    link: yew::ComponentLink<Self>,
    props: Properties,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Toggle => {
                self.props.active = !self.props.active;
                self.props.on_toggle.emit(self.props.active);
            }
        }

        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <div class=yew::classes!("form-check", "form-switch")>
                <input
                    id=self.props.id.clone()
                    type="checkbox"
                    class="form-check-input"
                    checked=self.props.active
                    onclick=self.link.callback(|_| Self::Message::Toggle)
                />
                <label class="form-check-label" for=self.props.id.clone()>{ &self.props.label }</label>
            </div>
        }
    }

    crate::change!(props.label, props.active, props.id);
}
