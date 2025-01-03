#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub value: String,
    #[prop_or_default]
    pub editable: bool,
    #[prop_or_default]
    pub on_click: yew::Callback<()>,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let bg_color = crate::cha::Color::from(&props.value);
    let color = if bg_color.is_dark() { "white" } else { "black" };
    let style = format!(
        "background-color: {}; color: {color}",
        bg_color.to_color_string(),
    );
    let on_click = {
        let on_click = props.on_click.clone();
        yew::Callback::from(move |_| on_click.emit(()))
    };

    yew::html! {
        <span {style} class="badge">
            { &props.value }
            {
                if props.editable {
                    yew::html! {
                        <crate::components::Svg
                            icon="x"
                            size=16
                            {on_click}
                        />
                    }
                } else {
                    "".into()
                }
            }
        </span>
    }
}
