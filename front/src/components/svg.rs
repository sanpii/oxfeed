#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub icon: String,
    pub size: u32,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub content_type: bool,
    #[prop_or_default]
    pub on_click: yew::Callback<()>,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let icon = if props.content_type {
        if props.icon.starts_with("audio/") {
            "file-earmark-music"
        } else if props.icon.starts_with("video/") {
            "file-earmark-play"
        } else {
            "file-earmark"
        }
    } else {
        &props.icon
    }
    .to_string();

    let span = gloo::utils::document().create_element("span").unwrap();

    let svg = format!(
        r#"
    <svg width={size} height={size} fill="currentColor">
        <use xlink:href="/bootstrap-icons.svg#{icon}"/>
    </svg>
    "#,
        size = props.size,
    );

    span.set_inner_html(&svg);

    let node = yew::virtual_dom::VNode::VRef(span.into());

    let onclick =
        yew_callback::callback!(on_click = props.on_click, move |e: web_sys::MouseEvent| {
            e.prevent_default();
            on_click.emit(())
        });

    yew::html! {
        <span class={ &props.class } {onclick}>{ node }</span>
    }
}
