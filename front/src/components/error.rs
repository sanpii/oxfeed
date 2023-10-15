#[derive(PartialEq, yew::Properties)]
pub struct Properties {
    pub text: String,
}

#[yew::function_component]
pub fn Component(props: &Properties) -> yew::Html {
    yew::html! {
        <span class="help">
            {" "}
            <crate::components::Svg icon="exclamation-octagon" size=16 class="text-danger" />
            <crate::components::Popover
                title={ "Last error".to_string() }
                text={ props.text.clone() }
                position="end"
            />
        </span>
    }
}
