#[derive(Clone, Copy, Default)]
enum Scene {
    Add,
    #[default]
    View,
}

#[yew::component]
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

    let on_save = yew_callback::callback!(webhooks, move |webhook: oxfeed::webhook::Entity| {
        let mut new_webhooks = (*webhooks).clone();

        if let Some(old) = new_webhooks.iter_mut().find(|x| x.id == webhook.id) {
            *old = webhook;
        } else {
            new_webhooks.insert(0, webhook);
        }

        new_webhooks.sort_by(|a, b| a.name.cmp(&b.name));

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
        if matches!(*scene, Scene::View) {
            <a
                class="btn btn-primary"
                title="Add"
                onclick={ on_add }
            >
                <crate::components::Svg icon="plus" size=24 />
                { "Add" }
            </a>
        }
        <ul class="list-group">
            if matches!(*scene, Scene::Add) {
                <crate::components::form::Webhook
                    webhook={ oxfeed::webhook::Entity::default() }
                    {on_cancel}
                    {on_submit}
                />
            }

            for webhook in webhooks.iter() {
                <crate::components::Webhook
                    value={ webhook.clone() }
                    on_delete={ on_delete.clone() }
                    on_save={ on_save.clone() }
                />
            }
        </ul>
        </>
    }
}
