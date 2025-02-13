#[yew::function_component]
pub(crate) fn Component() -> yew::Html {
    let context = crate::use_context();
    let context_dispatcher = context.dispatcher();

    yew::html! {
        for context.alerts.iter().enumerate().map(|(idx, alert)| {
            let context_dispatcher = context_dispatcher.clone();
            yew::html! {
                <div class={ yew::classes!("alert", format!("alert-{}", alert.severity()), "alert-dismissible") } role="alert">
                    { &alert.message }

                    <button
                        class="btn-close"
                        onclick={ move |_| context_dispatcher.dispatch(crate::Action::RemoveAlert(idx)) }
                    >
                    </button>
                </div>
            }
        })
    }
}
