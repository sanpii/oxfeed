#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    pub icon: String,
    pub size: u32,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub on_click: yew::Callback<()>,
}

#[yew::function_component]
pub fn Component(props: &Properties) -> yew::Html {
    let span = gloo::utils::document().create_element("span").unwrap();

    let svg = format!(
        r#"
    <svg width={size} height={size} fill="currentColor">
        <use xlink:href="/bootstrap-icons.svg#{src}"/>
    </svg>
    "#,
        size = props.size,
        src = props.icon,
    );

    span.set_inner_html(&svg);

    let node = yew::virtual_dom::VNode::VRef(span.into());

    let onclick = crate::cb!(props.on_click);

    yew::html! {
        <span class={ &props.class } {onclick}>{ node }</span>
    }
}
