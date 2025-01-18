#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub source: oxfeed::source::Entity,
    pub on_cancel: yew::Callback<()>,
    pub on_submit: yew::Callback<oxfeed::source::Entity>,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let active = yew::use_state(|| props.source.active);
    let title = yew::use_state(|| props.source.title.clone());
    let url = yew::use_state(|| props.source.url.clone());
    let tags = yew::use_state(|| props.source.tags.clone());
    let webhooks = yew::use_mut_ref(|| props.source.webhooks.clone());
    let all_webhooks = yew::use_state(Vec::new);

    {
        let all_webhooks = all_webhooks.clone();

        yew::use_state(|| {
            wasm_bindgen_futures::spawn_local(async move {
                let webhooks = crate::Api::webhooks_all().await.unwrap_or_default();
                all_webhooks.set(webhooks);
            })
        });
    }

    let edit_title = crate::components::edit_cb(title.clone());
    let edit_url = crate::components::edit_cb(url.clone());
    let toggle_active = {
        let active = active.clone();

        yew::Callback::from(move |value| {
            active.set(value);
        })
    };

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();

        yew::Callback::from(move |_| {
            on_cancel.emit(());
        })
    };

    let on_submit = {
        let active = active.clone();
        let on_submit = props.on_submit.clone();
        let title = title.clone();
        let url = url.clone();
        let source = props.source.clone();
        let tags = tags.clone();
        let webhooks = webhooks.clone();

        yew::Callback::from(move |_| {
            let mut source = source.clone();

            source.active = *active;
            source.title = (*title).clone();
            source.url = (*url).clone();
            source.tags = (*tags).clone();
            source.webhooks = (*webhooks).clone().into_inner();

            on_submit.emit(source);
        })
    };

    yew::html! {
        <form>
            <div class="row mb-3">
                <label class="col-sm-1 col-form-label" for="title">{ "Title" }</label>
                <div class="col-sm-11">
                    <input
                        class="form-control"
                        name="title"
                        value={ (*title).clone() }
                        oninput={ edit_title }
                    />
                    <small class="form-text text-body-secondary">{ "Leave empty to use the feed title." }</small>
                </div>
            </div>

            <div class="row mb-3">
                <label class="col-sm-1 col-form-label" for="url">{ "Feed URL" }</label>
                <div class="col-sm-11">
                    <input
                        class="form-control"
                        name="url"
                        required=true
                        value={ (*url).clone() }
                        oninput={ edit_url }
                    />
                </div>
            </div>

            <div class="row mb-3">
                <label class="col-sm-1 col-form-label" for="tags">{ "Tags" }</label>
                <div class="col-sm-11">
                    <super::Tags
                        values={ (*tags).clone() }
                        on_change={ yew::Callback::from(move |value| tags.set(value)) }
                    />
                </div>
            </div>

            <div class="row mb-3">
                <div class="col-sm-11 offset-sm-1">
                    <crate::components::Switch
                        id="active"
                        active={ *active }
                        on_toggle={ toggle_active }
                        label="active"
                    />
                </div>
            </div>

            {
                if all_webhooks.is_empty() {
                    yew::Html::default()
                } else {
                    yew::html! {
                        <div class="row mb-3">
                            <label class="col-sm-1 col-form-label" for="webhooks">{ "Webhooks" }</label>
                            <div class="col-sm-11">
                            {
                                for all_webhooks.clone().iter().map(move |webhook| {
                                    let id = webhook.id.unwrap_or_default();
                                    let active = webhooks.borrow().contains(&id);

                                    yew::html! {
                                        <crate::components::Switch
                                            id={ id.to_string() }
                                            label={ webhook.name.clone() }
                                            active={ active }
                                            on_toggle={
                                                let webhooks = webhooks.clone();

                                                yew::Callback::from(move |active| if active {
                                                    if !webhooks.borrow().contains(&id) {
                                                        webhooks.borrow_mut().push(id);
                                                    }
                                                } else {
                                                    webhooks.borrow_mut().retain(|x| x != &id);
                                                })
                                            }
                                        />
                                    }
                                })
                            }
                            </div>
                        </div>
                    }
                }
            }

            <a
                class={ yew::classes!("btn", "btn-primary") }
                title="Save"
                onclick={ on_submit }
            >
                <crate::components::Svg icon="check" size=24 />
                { "Save" }
            </a>

            <a
                class={ yew::classes!("btn", "btn-secondary") }
                title="Cancel"
                onclick={ on_cancel }
            >
                <crate::components::Svg icon="x" size=24 />
                { "Cancel" }
            </a>
        </form>
    }
}
