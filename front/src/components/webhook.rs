#[derive(Default)]
enum Scene {
    Edit,
    #[default]
    View,
}

#[derive(yew::Properties, Clone, PartialEq)]
pub(crate) struct Properties {
    pub value: oxfeed::webhook::Entity,
    #[prop_or_default]
    pub on_delete: yew::Callback<uuid::Uuid>,
    #[prop_or_default]
    pub on_save: yew::Callback<oxfeed::webhook::Entity>,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let scene = yew::use_state(Scene::default);
    let value = yew::use_memo(props.clone(), |props| props.value.clone());
    let on_save = yew::use_memo(props.clone(), |props| props.on_save.clone());

    let on_delete =
        yew_callback::callback!(context, on_delete = props.on_delete, value, move |_| {
            let context = context.clone();
            let message = format!("Would you like delete '{}' webhook?", value.name);

            if gloo::dialogs::confirm(&message) {
                let id = value.id.unwrap();
                yew::platform::spawn_local(async move {
                    crate::api::call!(context, webhooks_delete, &id);
                });

                on_delete.emit(id);
            }
        });

    let save = yew_callback::callback!(
        context,
        on_save,
        scene,
        move |webhook: oxfeed::webhook::Entity| {
            let context = context.clone();
            let on_save = on_save.clone();

            yew::platform::spawn_local(async move {
                let id = webhook.id.unwrap();

                crate::api::call!(context, webhooks_update, &id, &webhook);
                on_save.emit(webhook);
            });

            scene.set(Scene::View);
        }
    );

    match *scene {
        Scene::Edit => yew::html! {
            <super::form::Webhook
                webhook={ (*value).clone() }
                on_cancel={ yew_callback::callback!(move |_| scene.set(Scene::View)) }
                on_submit={ save }
            />
        },
        Scene::View => {
            let webhook = value.clone();

            yew::html! {
                <>
                    <div class="d-inline-flex">
                        { webhook.name.clone() }

                        if let Some(ref last_error) = webhook.last_error {
                            <super::Error text={ last_error.clone() } />
                        }
                    </div>

                    <div class="btn-group float-end">
                        <button
                            class="btn btn-primary"
                            title="Edit"
                            onclick={ yew_callback::callback!(move |_| scene.set(Scene::Edit)) }
                        >
                            <super::Svg icon="pencil-square" size=16 />
                        </button>
                        <button
                            class="btn btn-danger"
                            title="Delete"
                            onclick={ on_delete.clone() }
                        >
                            <super::Svg icon="trash" size=16 />
                        </button>
                    </div>
                </>
            }
        }
    }
}
