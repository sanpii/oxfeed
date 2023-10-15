#[derive(Clone, PartialEq, Eq, yew::Properties)]
pub struct Properties {
    #[prop_or_default]
    pub title: Option<String>,
    pub text: String,
    pub position: String,
}

#[yew::function_component]
pub fn Component(props: &Properties) -> yew::Html {
    let position_class = format!("bs-popover-{}", props.position);

    let span = gloo::utils::document().create_element("span").unwrap();
    span.set_inner_html(&props.text);
    let node = yew::virtual_dom::VNode::VRef(span.into());

    yew::html! {
        <div class={ yew::classes!("popover", position_class) }>
            {
                if let Some(title) = &props.title {
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
