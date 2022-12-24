#[derive(Clone)]
pub enum Message {
    Click,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    pub value: String,
    #[prop_or_default]
    pub editable: bool,
    #[prop_or_default]
    pub on_click: yew::Callback<()>,
}

pub struct Component {
    editable: bool,
    value: String,
    on_click: yew::Callback<()>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let props = ctx.props().clone();

        Self {
            editable: props.editable,
            value: props.value,
            on_click: props.on_click,
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Click => self.on_click.emit(()),
        }

        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let bg_color = crate::cha::Color::from(&self.value);
        let color = if bg_color.is_dark() { "white" } else { "black" };
        let style = format!(
            "background-color: {}; color: {color}",
            bg_color.to_color_string(),
        );

        yew::html! {
            <span style={ style } class="badge">
                { &self.value }
                {
                    if self.editable {
                        yew::html! {
                            <crate::components::Svg
                                icon="x"
                                size=16
                                on_click={ ctx.link().callback(move |_| Message::Click) }
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
