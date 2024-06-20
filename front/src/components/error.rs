#[derive(PartialEq, yew::Properties)]
pub struct Properties {
    pub text: String,
}

#[yew::function_component]
pub fn Component(props: &Properties) -> yew::Html {
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
