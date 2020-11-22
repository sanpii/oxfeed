#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub icon: String,
    pub size: u32,
}

pub(crate) struct Component(Properties);

impl yew::Component for Component {
    type Message = ();
    type Properties = Properties;

    fn create(props: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self(props)
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let span = yew::utils::document().create_element("span").unwrap();

        let svg = format!(r#"
        <svg width={size} height={size} fill="currentColor">
            <use xlink:href="/lib/bootstrap-icons/bootstrap-icons.svg#{src}"/>
        </svg>
        "#, size=self.0.size, src=self.0.icon);

        span.set_inner_html(&svg);

        yew::virtual_dom::VNode::VRef(span.into())
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.0 != props;

        self.0 = props;

        should_render
    }
}
