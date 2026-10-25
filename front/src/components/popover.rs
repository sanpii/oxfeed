#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    pub body: String,
    pub children: yew::Html,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    yew::html! {
        <button
            class={ yew::classes!("btn-solid", "btn-sm", props.class.clone()) }
            data-bs-toggle="popover"
            data-bs-trigger="focus"
            data-bs-html="true"
            data-bs-title={ props.title.clone() }
            data-bs-content={ props.body.clone() }
        >
            { props.children.clone() }
        </button>
    }
}
