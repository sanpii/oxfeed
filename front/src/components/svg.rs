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
    link: yew::ComponentLink<Self>,
    on_click: yew::Callback<()>,
    size: u32,
    class: String,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            icon: props.icon,
            link,
            on_click: props.on_click,
            size: props.size,
            class: props.class,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Click => self.on_click.emit(()),
        }

        false
    }

    fn view(&self) -> yew::Html {
        let span = yew::utils::document().create_element("span").unwrap();

        let svg = format!(
            r#"
        <svg width={size} height={size} fill="currentColor">
            <use xlink:href="/lib/bootstrap-icons/bootstrap-icons.svg#{src}"/>
        </svg>
        "#,
            size = self.size,
            src = self.icon,
        );

        span.set_inner_html(&svg);

        let node = yew::virtual_dom::VNode::VRef(span.into());

        yew::html! {
            <span class=&self.class onclick=self.link.callback(|_| Self::Message::Click)>
            { node }
            </span>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.icon != props.icon
            || self.size != props.size
            || self.class != props.class;

        self.icon = props.icon;
        self.size = props.size;
        self.class = props.class;
        self.on_click = props.on_click;

        should_render
    }
}
