#[derive(Clone, Copy, Default)]
enum Scene {
    Add,
    #[default]
    View,
}

#[yew::function_component]
pub(crate) fn Component() -> yew::Html {
    let context = crate::use_context();
    let scene = yew::use_state(Scene::default);
    let webhooks = yew::use_state(Vec::new);

    {
        let context = context.clone();
        let webhooks = webhooks.clone();

        yew::use_effect_with((), move |_| {
            let context = context.clone();
            let webhooks = webhooks.clone();

            yew::platform::spawn_local(async move {
                let new_webhooks = crate::api::call!(context, webhooks_all);
                webhooks.set(new_webhooks);
            });
        });
    }

    let on_add = yew_callback::callback!(scene, move |_| {
        scene.set(Scene::Add);
    });

    let on_cancel = yew_callback::callback!(scene, move |_| {
        scene.set(Scene::View);
    });

    let on_delete = yew_callback::callback!(webhooks, move |id| {
        let new_webhooks = webhooks
            .iter()
            .filter(|x| x.id != Some(id))
            .cloned()
            .collect();
        webhooks.set(new_webhooks);
    });

    let on_save = yew_callback::callback!(webhooks, move |webhook| {
        let mut new_webhooks = (*webhooks).clone();
        new_webhooks.insert(0, webhook);
        webhooks.set(new_webhooks);
    });

    let on_submit = yew_callback::callback!(context, on_save, scene, move |webhook| {
        let context = context.clone();
        let on_save = on_save.clone();

        yew::platform::spawn_local(async move {
            crate::api::call!(context, webhooks_create, &webhook);
            on_save.emit(webhook);
        });

        scene.set(Scene::View);
    });

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
                            on_save={ on_save.clone() }
                        />
                    </li>
                }
            })
        }
        </ul>
        </>
    }
}
