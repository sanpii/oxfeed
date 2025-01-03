#[derive(Clone, Copy, Default)]
enum Scene {
    Add,
    #[default]
    View,
}

#[yew::function_component]
pub(crate) fn Component() -> yew::HtmlResult {
    let force_reload = yew::use_state(|| 0);
    let scene = yew::use_state(Scene::default);
    let webhooks = yew::suspense::use_future_with(force_reload.clone(), |_| async move {
        crate::Api::webhooks_all().await.unwrap_or_default()
    })?;

    let on_add = {
        let scene = scene.clone();

        yew::Callback::from(move |_| {
            scene.set(Scene::Add);
        })
    };

    let on_cancel = {
        let scene = scene.clone();

        yew::Callback::from(move |_| {
            scene.set(Scene::View);
        })
    };

    let on_delete = {
        let force_reload = force_reload.clone();

        yew::Callback::from(move |_| {
            force_reload.set(*force_reload + 1);
        })
    };

    let on_submit = {
        let force_reload = force_reload.clone();
        let scene = scene.clone();

        yew::Callback::from(move |webhook| {
            let force_reload = force_reload.clone();

            yew::suspense::Suspension::from_future(async move {
                crate::Api::webhooks_create(&webhook).await.unwrap();
                force_reload.set(*force_reload + 1);
            });

            scene.set(Scene::View);
        })
    };

    let html = yew::html! {
        <>
        {
            if matches!(*scene, Scene::View) {
                yew::html! {
                    <a
                        class={ yew::classes!("btn", "btn-primary") }
                        title="Add"
                        onclick={ on_add }
                    >
                        <crate::components::Svg icon="plus" size=24 />
                        { "Add" }
                    </a>
                }
            } else {
                "".into()
            }
        }
        <ul class="list-group">
        {
            if matches!(*scene, Scene::Add) {
                yew::html! {
                    <li class="list-group-item">
                        <crate::components::form::Webhook
                            webhook={ oxfeed_common::webhook::Entity::default() }
                            {on_cancel}
                            {on_submit}
                        />
                    </li>
                }
            } else {
                "".into()
            }
        }
        {
            for webhooks.clone().iter().map(|webhook| {
                yew::html!{
                    <li class="list-group-item">
                        <crate::components::Webhook
                            value={ webhook.clone() }
                            on_delete={ on_delete.clone() }
                        />
                    </li>
                }
            })
        }
        </ul>
        </>
    };

    Ok(html)
}
