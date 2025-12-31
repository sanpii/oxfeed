#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub button: bool,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let current_theme = yew::use_memo(context.clone(), |context| context.theme);

    if props.button {
        yew::html! {
            <li class="nav-item dropdown d-none d-md-block mx-2">
                <button class="btn btn-dark dropdown-toggle p-2" data-bs-toggle="dropdown">
                    <super::Svg icon={ current_theme.icon() } size=16 />
                </button>
                <Menu current_theme={ *current_theme } dark=true />
            </li>
        }
    } else {
        yew::html! {
            <>
                <a class="nav-link dropdown-toggle" data-bs-toggle="dropdown" data-bs-target="theme" href="#">
                    <super::Svg icon={ current_theme.icon() } size=16 />
                    { "Theme" }
                </a>
                <Menu current_theme={ *current_theme } dark=false />
            </>
        }
    }
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct MenuProperties {
    pub current_theme: crate::context::Theme,
    pub dark: bool,
}

#[yew::component]
fn Menu(props: &MenuProperties) -> yew::Html {
    let context = crate::use_context();
    let on_click = yew_callback::callback!(context, move |value: crate::context::Theme| {
        use gloo::storage::Storage as _;

        match gloo::storage::LocalStorage::set("theme", value) {
            Ok(_) => context.dispatch(crate::Action::Theme(value)),
            Err(err) => context.dispatch(oxfeed::Error::from(err).into()),
        }
    });

    yew::html! {
        <ul class={ yew::classes!("dropdown-menu", if props.dark { "dropdown-menu-dark" } else { "" }) } id="theme">
            for theme in crate::context::Theme::all() {
                <li>
                    <a
                        class={ yew::classes!("dropdown-item", if props.current_theme == theme { "active" } else { "" }) }
                        onclick={
                            let on_click = on_click.clone();

                            move |_| on_click.emit(theme)
                        }
                    >
                        <super::Svg icon={ theme.icon() } size=16 />{ theme }
                    </a>
                </li>
            }
        </ul>
    }
}
