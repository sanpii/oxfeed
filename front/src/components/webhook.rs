#[derive(Default)]
enum Scene {
    Edit,
    #[default]
    View,
}

#[derive(yew::Properties, Clone, PartialEq)]
pub(crate) struct Properties {
    pub value: oxfeed_common::webhook::Entity,
    #[prop_or_default]
    pub on_delete: yew::Callback<()>,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let scene = yew::use_state(Scene::default);
    let value = yew::use_state(|| props.value.clone());

    let on_delete = {
        let value = value.clone();
        let on_delete = props.on_delete.clone();

        yew::Callback::from(move |_| {
            let message = format!("Would you like delete '{}' webhook?", value.name);

            if gloo::dialogs::confirm(&message) {
                let id = value.id.unwrap();
                yew::suspense::Suspension::from_future(async move {
                    crate::Api::webhooks_delete(&id).await.unwrap();
                });

                on_delete.emit(());
            }
        })
    };

    let save = {
        let scene = scene.clone();
        let value = value.clone();

        yew::Callback::from(move |webhook: oxfeed_common::webhook::Entity| {
            let id = webhook.id.unwrap();
            value.set(webhook.clone());

            yew::suspense::Suspension::from_future(async move {
                crate::Api::webhooks_update(&id, &webhook).await.unwrap();
            });

            scene.set(Scene::View);
        })
    };

    match *scene {
        Scene::Edit => yew::html! {
            <super::form::Webhook
                webhook={ (*value).clone() }
                on_cancel={ yew::Callback::from(move |_| scene.set(Scene::View)) }
                on_submit={ save }
            />
        },
        Scene::View => {
            let webhook = value.clone();

            yew::html! {
                <>
                    <div class="d-inline-flex">
                    { webhook.name.clone() }
                    {
                        if let Some(ref last_error) = webhook.last_error {
                            yew::html! {
                                <super::Error text={ last_error.clone() } />
                            }
                        }
                        else {
                            "".into()
                        }
                    }
                    </div>

                    <div class={ yew::classes!("btn-group", "float-end") }>
                        <button
                            class={ yew::classes!("btn", "btn-primary") }
                            title="Edit"
                            onclick={ yew::Callback::from(move |_| scene.set(Scene::Edit)) }
                        >
                            <super::Svg icon="pencil-square" size=16 />
                        </button>
                        <button
                            class={ yew::classes!("btn", "btn-danger") }
                            title="Delete"
                            onclick={ on_delete }
                        >
                            <super::Svg icon="trash" size=16 />
                        </button>
                    </div>
                </>
            }
        }
    }
}
