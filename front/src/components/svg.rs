#[derive(yew::Properties, Clone)]
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
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let span = document.create_element("span").unwrap();

        let svg = format!(r#"
        <svg width={size} height={size} fill="currentColor">
            <use xlink:href="/lib/bootstrap-icons/bootstrap-icons.svg#{src}"/>
        </svg>
        "#, size=self.0.size, src=self.0.icon);

        span.set_inner_html(&svg);

        yew::virtual_dom::VNode::VRef(span.into())
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}

