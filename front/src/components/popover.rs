#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    pub body: yew::Html,
    pub children: yew::Html,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let id = format!("popover-{}", uuid::Uuid::new_v4());

    yew::html! {
        <>
            <button
                class={ yew::classes!("btn", "btn-sm", props.class.clone()) }
                popovertarget={ id.clone() }
                popovertargetaction="toggle"
            >
                { props.children.clone() }
            </button>
            <div id={ id.clone() } class="popover" popover="auto">
                if let Some(title) = &props.title {
                    <h3 class="popover-header">{ title }</h3>
                }
                <div class="popover-body">{ props.body.clone() }</div>
            </div>
        </>
    }
}
