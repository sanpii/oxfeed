#[derive(Clone, Copy, Default)]
enum Scene {
    Add,
    #[default]
    View,
}

#[yew::function_component]
pub(crate) fn Component() -> yew::Html {
    let context = crate::use_context();
    let force_reload = yew::use_state(|| 0);
    let scene = yew::use_state(Scene::default);
    let webhooks = yew::use_state(Vec::new);

    {
        let context = context.clone();
        let webhooks = webhooks.clone();

        yew::use_effect_with(force_reload.clone(), move |_| {
            let context = context.clone();
            let webhooks = webhooks.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let new_webhooks = crate::api::call!(context, webhooks_all);
                webhooks.set(new_webhooks);
            });
        });
    }

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
        let context = context.clone();
        let force_reload = force_reload.clone();
        let scene = scene.clone();

        yew::Callback::from(move |webhook| {
            let context = context.clone();
            let force_reload = force_reload.clone();

            wasm_bindgen_futures::spawn_local(async move {
                crate::api::call!(context, webhooks_create, &webhook);
                force_reload.set(*force_reload + 1);
            });

            scene.set(Scene::View);
        })
    };

    yew::html! {
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
                yew::Html::default()
            }
        }
        <ul class="list-group">
        {
            if matches!(*scene, Scene::Add) {
                yew::html! {
                    <li class="list-group-item">
                        <crate::components::form::Webhook
                            webhook={ oxfeed::webhook::Entity::default() }
                            {on_cancel}
                            {on_submit}
                        />
                    </li>
                }
            } else {
                yew::Html::default()
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
    }
}
