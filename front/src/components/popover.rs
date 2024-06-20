#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    #[prop_or_default]
    pub title: Option<String>,
    pub body: yew::Html,
    pub children: yew::Html,
}

#[yew::function_component]
pub fn Component(props: &Properties) -> yew::Html {
    let id = format!("popover-{}", uuid::Uuid::new_v4());

    yew::html! {
        <>
            <button
                class="btn btn-sm"
                popovertarget={ id.clone() }
                popovertargetaction="show"
                style="padding: 0; margin: 0;"
            >
                { props.children.clone() }
            </button>
            <div id={ id.clone() } class="popover" popover="auto">
                {
                    if let Some(title) = &props.title {
                        yew::html! {
                            <h3 class="popover-header">{ title }</h3>
                        }
                    } else {
                        "".into()
                    }
                }
                <div class="popover-body">{ props.body.clone() }</div>
            </div>
        </>
    }
}
