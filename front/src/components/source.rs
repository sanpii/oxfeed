#[derive(Clone, Copy, Default)]
enum Scene {
    Edit,
    #[default]
    View,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub value: oxfeed::source::Entity,
    pub webhooks: Vec<oxfeed::webhook::Entity>,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let source = yew::use_memo(props.clone(), |props| props.value.clone());
    let scene = yew::use_state(Scene::default);

    let on_cancel = yew_callback::callback!(scene, move |_| {
        scene.set(Scene::View);
    });

    let on_delete = yew_callback::callback!(source, context, move |_| {
        let message = format!("Would you like delete '{}' source?", source.title);

        if gloo::dialogs::confirm(&message) {
            let source = source.clone();
            let context = context.clone();

            yew::platform::spawn_local(async move {
                let id = source.id.unwrap();

                crate::api::call!(context, sources_delete, &id);
                context.dispatch(crate::Action::NeedUpdate);
            });
        }
    });

    let on_edit = yew_callback::callback!(scene, move |_| {
        scene.set(Scene::Edit);
    });

    let on_submit =
        yew_callback::callback!(context, scene, move |new_source: oxfeed::source::Entity| {
            let context = context.clone();
            let scene = scene.clone();

            yew::platform::spawn_local(async move {
                let id = new_source.id.unwrap();

                crate::api::call!(context, sources_update, &id, &new_source);
                scene.set(Scene::View);
                context.dispatch(crate::Action::NeedUpdate);
            });
        });

    let on_toggle = yew_callback::callback!(source, on_submit, move |active| {
        let mut new_source = (*source).clone();

        new_source.active = active;
        on_submit.emit(new_source);
    });

    let source = (*source).clone();

    let icon = source
        .icon
        .clone()
        .unwrap_or_else(|| "/1px.png".to_string());

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

                        <span>
                            <img src={ icon } width="16" height="16" class="me-1" />
                            { source.title }
                        </span>

                        if let Some(last_error) = source.last_error {
                            <super::Error text={ last_error } />
                        }
                    </div>

                    <div class="btn-group float-end">
                        {
                            if source.webhooks.is_empty() {
                                yew::Html::default()
                            } else {
                                let body = yew::html! {
                                    <ul>
                                        for webhook_id in &source.webhooks {
                                            if let Some(w) = props.webhooks.iter().find(|x| x.id == Some(*webhook_id)) {
                                                <li>{ w.name.clone() }</li>
                                            } else {
                                                <></>
                                            }
                                        }
                                    </ul>
                                };

                                yew::html! {
                                    <super::Popover {body} class="btn-warning">
                                        <super::Svg icon="plug" size=16 />
                                    </super::Popover>
                                }
                            }
                        }
                        <button
                            class="btn btn-primary"
                            title="Edit"
                            onclick={ on_edit }
                        >
                            <super::Svg icon="pencil-square" size=16 />
                        </button>
                        <button
                            class="btn btn-danger"
                            title="Delete"
                            onclick={ on_delete }
                        >
                            <super::Svg icon="trash" size=16 />
                        </button>
                    </div>

                    <div class="tags">
                        for tag in source.tags {
                            <super::Tag value={ tag.clone() } />
                        }
                    </div>
                </>
            }
        }
    }
}
