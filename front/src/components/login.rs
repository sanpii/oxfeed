#[derive(Clone, Copy, Default)]
enum Scene {
    #[default]
    Login,
    Register,
}

#[yew::function_component]
pub(crate) fn Component() -> yew::Html {
    let context = crate::use_context();
    let scene = yew::use_state(Scene::default);

    let on_cancel = {
        let scene = scene.clone();

        yew::Callback::from(move |_| {
            scene.set(Scene::Login);
        })
    };

    let on_create = {
        let context = context.clone();
        let scene = scene.clone();

        yew::Callback::from(move |info: crate::components::form::register::Info| {
            let context = context.clone();
            let scene = scene.clone();
            let user = oxfeed_common::account::Entity {
                id: None,
                password: info.password,
                email: info.email,
            };

            wasm_bindgen_futures::spawn_local(async move {
                match crate::Api::account_create(&user).await {
                    Ok(_) => {
                        let alert = crate::Alert::info("User created");
                        context.dispatch(alert.into());
                        scene.set(Scene::Login);
                    }
                    Err(err) => context.dispatch(err.into()),
                }
            });
        })
    };

    let on_login = {
        let context = context.clone();

        yew::Callback::from(move |info: crate::components::form::login::Info| {
            let context = context.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match crate::Api::auth_login(&info.email, &info.password, &info.remember_me).await {
                    Ok(_) => context.dispatch(crate::Action::Logged),
                    Err(err) => context.dispatch(err.into()),
                }
            });
        })
    };

    let on_register = {
        let scene = scene.clone();

        yew::Callback::from(move |_| {
            scene.set(Scene::Register);
        })
    };

    match *scene {
        Scene::Login => yew::html! {
            <div class="login">
                <form>
                    <img class="mb-4" src="/logo.png" alt="" width="72px" height="72px" />
                    <h1 class="h3 mb-3 fw-normal">{ "Please sign in" }</h1>
                    <super::Alerts />
                    <super::form::Login on_submit={ on_login } />
                    { "Don't have an account yet?" }
                    <a href="#" onclick={ on_register }>{ "Register now" }</a>
                </form>
            </div>
        },
        Scene::Register => yew::html! {
            <div class="login">
                <form>
                    <img class="mb-4" src="/logo" alt="" width="72px" height="72px" />
                    <h1 class="h3 mb-3 fw-normal">{ "Register" }</h1>
                    <super::Alerts />
                    <super::form::Register on_submit={ on_create } />
                    { "Already have login and password?" }
                    <a href="#" onclick={ on_cancel }>{ "Log in" }</a>
                </form>
            </div>
        },
    }
}
