#[derive(Clone)]
pub(crate) enum Message {
    Click,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub value: String,
    #[prop_or_default]
    pub editable: bool,
    #[prop_or_default]
    pub on_click: yew::Callback<()>,
}

pub(crate) struct Component {
    editable: bool,
    link: yew::ComponentLink<Self>,
    value: String,
    on_click: yew::Callback<()>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            editable: props.editable,
            link,
            value: props.value,
            on_click: props.on_click,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Click => self.on_click.emit(()),
        }

        false
    }

    fn view(&self) -> yew::Html {
        let bg_color = crate::cha::Color::from(&self.value);
        let color = if bg_color.is_dark() { "white" } else { "black" };
        let style = format!(
            "background-color: {}; color: {}",
            bg_color.to_color_string(),
            color
        );

        yew::html! {
            <span style=style class="badge">
                { &self.value }
                {
                    if self.editable {
                        yew::html! {
                            <crate::components::Svg
                                icon="x"
                                size=16
                                on_click=self.link.callback(move |_| Self::Message::Click)
                            />
                        }
                    } else {
                        "".into()
                    }
                }
            </span>
        }
    }

    crate::change!(editable, value, on_click);
}
