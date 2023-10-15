#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    pub id: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub on_toggle: yew::Callback<bool>,
}

#[yew::function_component]
pub fn Component(props: &Properties) -> yew::Html {
    let checked = yew::functional::use_state_eq(|| props.active);

    let onclick = {
        let props = props.clone();
        let checked = checked.clone();
        yew::Callback::from(move |_| {
            checked.set(!*checked);
            props.on_toggle.emit(!*checked);
        })
    };

    yew::html! {
        <div class={ yew::classes!("form-check", "form-switch") }>
            <input
                id={ props.id.clone() }
                type="checkbox"
                class="form-check-input"
                checked={ *checked }
                {onclick}
            />
            <label class="form-check-label" for={ props.id.clone() }>{ &props.label }</label>
        </div>
    }
}
