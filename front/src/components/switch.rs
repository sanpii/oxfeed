#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub id: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub on_toggle: yew::Callback<bool>,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let onclick = yew_callback::callback!(
        on_toggle = props.on_toggle,
        active = props.active,
        move |_| {
            on_toggle.emit(!active);
        }
    );

    yew::html! {
        <div class="form-check form-switch">
            <input
                id={ props.id.clone() }
                type="checkbox"
                class="form-check-input"
                checked={ props.active }
                {onclick}
            />
            <label class="form-check-label" for={ props.id.clone() }>{ &props.label }</label>
        </div>
    }
}
