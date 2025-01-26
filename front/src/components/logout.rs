#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub button: bool,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let navigator = yew_router::hooks::use_navigator().unwrap();

    let logout = yew_callback::callback!(context, navigator, move |_| {
        let context = context.clone();
        let navigator = navigator.clone();

        yew::platform::spawn_local(async move {
            crate::api::call!(context, auth_logout);
            context.dispatch(crate::Action::AuthRequire);

            let alert = crate::Alert::info("Logged out");
            context.dispatch(alert.into());
            navigator.push(&crate::components::app::Route::Index);
        });
    });

    if props.button {
        yew::html! {
                <button
                    class={ yew::classes!("btn", "btn-secondary", "logout", "d-none", "d-md-block") }
                    title="Logout"
                    onclick={ logout }
                >
                    <super::Svg icon="door-closed" size=24 />
                </button>
        }
    } else {
        yew::html! {
            <a class="nav-link" onclick={ logout } href="#">
                <super::Svg icon="door-closed" size=24 /> { "Logout" }
            </a>
        }
    }
}
