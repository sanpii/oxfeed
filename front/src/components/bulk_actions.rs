pub enum Selection {
    All,
    None,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub active: bool,
    pub on_action: yew::Callback<(&'static str, bool)>,
    pub on_select: yew::Callback<Selection>,
    pub on_toggle: yew::Callback<bool>,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let on_toggle = yew_callback::callback!(
        active = props.active,
        on_toggle = props.on_toggle,
        move |_| {
            on_toggle.emit(!active);
        }
    );

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

    let on_selectall = yew_callback::callback!(on_select = props.on_select, move |_| {
        on_select.emit(Selection::All);
    });
    let on_unselectall = yew_callback::callback!(on_select = props.on_select, move |_| {
        on_select.emit(Selection::None);
    });

    yew::html! {
        <div class="input-group mb-3">
            <div class="btn-group">
                <div class="btn btn-outline-secondary">
                    <input type="checkbox" class="form-check-input" checked={ props.active } onclick={ on_toggle } />
                </div>

                <button type="button" class="btn btn-outline-secondary dropdown-toggle dropdown-toggle-split" data-bs-toggle="dropdown">
                </button>
                <ul class="dropdown-menu">
                    <li><a class="dropdown-item" href="#" onclick={ move |_| on_selectall.emit(()) }>{ "All" }</a></li>
                    <li><a class="dropdown-item" href="#" onclick={ move |_| on_unselectall.emit(()) }>{ "None" }</a></li>
                </ul>
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
