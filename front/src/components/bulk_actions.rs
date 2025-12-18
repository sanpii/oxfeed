#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub active: bool,
    pub on_action: yew::Callback<(&'static str, bool)>,
    pub on_toggle: yew::Callback<bool>,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let on_toggle = yew_callback::callback!(on_toggle = props.on_toggle, move |enabled| {
        on_toggle.emit(enabled);
    });

    let on_read = yew_callback::callback!(on_action = props.on_action, move |_| {
        on_action.emit(("read", true));
    });
    let on_unread = yew_callback::callback!(on_action = props.on_action, move |_| {
        on_action.emit(("read", false));
    });
    let on_favorite = yew_callback::callback!(on_action = props.on_action, move |_| {
        on_action.emit(("favorite", true));
    });
    let on_unfavorite = yew_callback::callback!(on_action = props.on_action, move |_| {
        on_action.emit(("favorite", false));
    });

    yew::html! {
        <div class="input-group mb-3">
            <div class="btn-group">
                <div class="btn btn-outline-secondary">
                    <super::Switch id="enable-bulk" {on_toggle} active={ props.active } />
                </div>
            </div>

            <div class="btn-group mx-2">
                <button type="button" class="btn btn-outline-primary" onclick={ on_read }>
                    <super::Svg icon="eye" size=24 />
                </button>
                <button type="button" class="btn btn-outline-primary" onclick={ on_unread }>
                    <super::Svg icon="eye-slash" size=24 />
                </button>
            </div>

            <div class="btn-group">
                <button type="button" class="btn btn-outline-warning" onclick={ on_favorite }>
                    <super::Svg icon="star-fill" size=24 />
                </button>
                <button type="button" class="btn btn-outline-warning" onclick={ on_unfavorite }>
                    <super::Svg icon="star" size=24 />
                </button>
            </div>
        </div>
    }
}
