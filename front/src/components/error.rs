#[derive(PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub text: String,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    yew::html! {
        <span class="help">
            {" "}
            <super::Popover
                title={ "Last error".to_string() }
                body={ props.text.clone() }
            >
                <super::Svg icon="exclamation-octagon" size=16 class="text-danger" />
            </super::Popover>
        </span>
    }
}
