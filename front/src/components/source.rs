#[derive(Clone, Copy, Default)]
enum Scene {
    Edit,
    #[default]
    View,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub value: oxfeed_common::source::Entity,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let source = yew::use_state(|| props.value.clone());
    let scene = yew::use_state(Scene::default);

    let on_cancel = {
        let scene = scene.clone();

        yew::Callback::from(move |_| {
            scene.set(Scene::View);
        })
    };

    let on_delete = {
        let source = source.clone();
        let context = context.clone();

        yew::Callback::from(move |_| {
            let message = format!("Would you like delete '{}' source?", source.title);

            if gloo::dialogs::confirm(&message) {
                let source = source.clone();
                let context = context.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let id = source.id.unwrap();

                    crate::Api::sources_delete(&id).await.unwrap();
                    context.dispatch(crate::Action::NeedUpdate);
                });
            }
        })
    };

    let on_edit = {
        let scene = scene.clone();

        yew::Callback::from(move |_| {
            scene.set(Scene::Edit);
        })
    };

    let on_submit = {
        let scene = scene.clone();
        let source = source.clone();

        yew::Callback::from(move |new_source: oxfeed_common::source::Entity| {
            let scene = scene.clone();
            let source = source.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let id = new_source.id.unwrap();

                crate::Api::sources_update(&id, &new_source).await.unwrap();
                source.set(new_source);
                scene.set(Scene::View);
            });
        })
    };

    let on_toggle = {
        let source = source.clone();
        let on_submit = on_submit.clone();

        yew::Callback::from(move |active| {
            let mut new_source = (*source).clone();

            new_source.active = active;
            on_submit.emit(new_source);
        })
    };

    let source = (*source).clone();

    match *scene {
        Scene::Edit => yew::html! {
            <super::form::Source {source} {on_cancel} {on_submit} />
        },
        Scene::View => {
            yew::html! {
                <>
                    <div class="d-inline-flex">
                        <super::Switch
                            id={ format!("active-{}", source.id.unwrap_or_default()) }
                            active={ source.active }
                            {on_toggle}
                        />

                        { source.title }

                        {
                            if let Some(last_error) = source.last_error {
                                yew::html! {
                                    <super::Error text={ last_error } />
                                }
                            }
                            else {
                                "".into()
                            }
                        }
                    </div>

                    <div class={ yew::classes!("btn-group", "float-end") }>
                        {
                            if source.webhooks.is_empty() {
                                "".into()
                            } else {
                                yew::html! {
                                    <button class={ yew::classes!("btn", "btn-warning") } disabled=true>
                                        <super::Svg icon="plug" size=16 />
                                    </button>
                                }
                            }
                        }
                        <button
                            class={ yew::classes!("btn", "btn-primary") }
                            title="Edit"
                            onclick={ on_edit }
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

                    <div class="tags">
                    {
                        for source.tags.iter().map(|tag| {
                            yew::html! { <super::Tag value={ tag.clone() } /> }
                        })
                    }
                    </div>
                </>
            }
        }
    }
}
