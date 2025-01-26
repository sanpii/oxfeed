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

    let on_cancel = yew_callback::callback!(scene, move |_| {
        scene.set(Scene::Login);
    });

    let on_create = yew_callback::callback!(
        context,
        scene,
        move |info: crate::components::form::register::Info| {
            let context = context.clone();
            let scene = scene.clone();
            let user = oxfeed::account::Entity {
                id: None,
                password: info.password,
                email: info.email,
            };

            yew::platform::spawn_local(async move {
                crate::api::call!(context, account_create, &user);

                let alert = crate::Alert::info("User created");
                context.dispatch(alert.into());
                scene.set(Scene::Login);
            });
        }
    );

    let on_login = yew_callback::callback!(
        context,
        move |info: crate::components::form::login::Info| {
            let context = context.clone();

            yew::platform::spawn_local(async move {
                crate::api::call!(
                    context,
                    auth_login,
                    &info.email,
                    &info.password,
                    &info.remember_me
                );
                context.dispatch(crate::Action::Logged);
            });
        }
    );

    let on_register = yew_callback::callback!(scene, move |_| {
        scene.set(Scene::Register);
    });

    match *scene {
        Scene::Login => yew::html! {
            <div class="login">
                <form>
                    <super::Svg icon="rss" size=72 />
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
                    <super::Svg icon="rss" size=72 />
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
