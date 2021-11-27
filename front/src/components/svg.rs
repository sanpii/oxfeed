pub(crate) enum Message {
    Click,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub icon: String,
    pub size: u32,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub on_click: yew::Callback<()>,
}

pub(crate) struct Component {
    icon: String,
    on_click: yew::Callback<()>,
    size: u32,
    class: String,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let props = ctx.props().clone();

        Self {
            icon: props.icon,
            on_click: props.on_click,
            size: props.size,
            class: props.class,
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Click => self.on_click.emit(()),
        }

        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let span = gloo::utils::document().create_element("span").unwrap();

        let svg = format!(
            r#"
        <svg width={size} height={size} fill="currentColor">
            <use xlink:href="/bootstrap-icons.svg#{src}"/>
        </svg>
        "#,
            size = self.size,
            src = self.icon,
        );

        span.set_inner_html(&svg);

        let node = yew::virtual_dom::VNode::VRef(span.into());

        yew::html! {
            <span class={ &self.class } onclick={ ctx.link().callback(|_| Message::Click) }>
            { node }
            </span>
        }
    }

    crate::change!(icon, size, class, on_click);
}
