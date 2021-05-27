#[derive(Clone, PartialEq, Eq, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub title: Option<String>,
    pub text: String,
    pub position: String,
}

pub(crate) struct Component {
    props: Properties,
}

impl yew::Component for Component {
    type Message = ();
    type Properties = Properties;

    fn create(props: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let position_class = format!("bs-popover-{}", self.props.position);

        let span = yew::utils::document().create_element("span").unwrap();
        span.set_inner_html(&self.props.text);
        let node = yew::virtual_dom::VNode::VRef(span.into());

        yew::html! {
            <div class=yew::classes!("popover", position_class)>
                {
                    if let Some(title) = &self.props.title {
                        yew::html! {
                            <div class="popover-header">{ title }</div>
                        }
                    } else {
                        "".into()
                    }
                }
                <div class="popover-arrow"></div>
                <div class="popover-body">{ node }</div>
            </div>
        }
    }

    crate::change!(props);
}
