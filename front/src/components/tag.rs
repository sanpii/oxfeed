#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub value: String,
    #[prop_or_default]
    pub deletable: bool,
    #[prop_or_default]
    pub editable: bool,
    #[prop_or_default]
    pub on_delete: yew::Callback<()>,
    #[prop_or_default]
    pub on_edit: yew::Callback<()>,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let bg_color = crate::cha::Color::from(&props.value);
    let color = if bg_color.is_dark() { "white" } else { "black" };
    let style = format!(
        "background-color: {}; color: {color}",
        bg_color.to_color_string(),
    );

    let on_delete =
        yew_callback::callback!(on_delete = props.on_delete, move |_| on_delete.emit(()));
    let on_edit = yew_callback::callback!(on_edit = props.on_edit, move |_| on_edit.emit(()));

    yew::html! {
        <span {style} class="badge position-relative">
            { &props.value }
            {
                if props.deletable {
                    yew::html! {
                        <crate::components::Svg
                            icon="x"
                            size=16
                            on_click={ on_delete }
                        />
                    }
                } else if props.editable {
                    yew::html! {
                        <span class="position-absolute top-1 start-99" style="font-size: 1rem;">
                            <crate::components::Svg
                                icon="pencil"
                                size=16
                                on_click={ on_edit }
                            />
                        </span>
                    }
                } else {
                    yew::Html::default()
                }
            }
        </span>
    }
}
