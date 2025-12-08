#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub source: oxfeed::source::Entity,
    pub on_cancel: yew::Callback<()>,
    pub on_submit: yew::Callback<oxfeed::source::Entity>,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let active = yew::use_state(|| props.source.active);
    let title = yew::use_state(|| props.source.title.clone());
    let url = yew::use_state(|| props.source.url.clone());
    let tags = yew::use_state(|| props.source.tags.clone());
    let webhooks = yew::use_mut_ref(|| props.source.webhooks.clone());
    let all_webhooks = yew::use_state(Vec::new);

    {
        let all_webhooks = all_webhooks.clone();

        yew::use_state(|| {
            let context = context.clone();

            yew::platform::spawn_local(async move {
                let webhooks = crate::api::call!(context, webhooks_all);
                all_webhooks.set(webhooks);
            })
        });
    }

    let edit_title = crate::components::edit_cb(title.clone());
    let edit_url = crate::components::edit_cb(url.clone());
    let toggle_active = yew_callback::callback!(active, move |value| {
        active.set(value);
    });

    let on_cancel = yew_callback::callback!(on_cancel = props.on_cancel, move |_| {
        on_cancel.emit(());
    });

    let on_submit = yew_callback::callback!(
        active,
        on_submit = props.on_submit,
        title,
        url,
        source = props.source,
        tags,
        webhooks,
        move |_| {
            let mut source = source.clone();

            source.active = *active;
            source.title = (*title).clone();
            source.url = (*url).clone();
            source.tags = (*tags).clone();
            source.webhooks = (*webhooks).clone().into_inner();

            on_submit.emit(source);
        }
    );

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
                        on_change={ yew_callback::callback!(move |value| tags.set(value)) }
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

            if !all_webhooks.is_empty() {
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
                                        yew_callback::callback!(webhooks,
                                            move |active| if active {
                                                if !webhooks.borrow().contains(&id) {
                                                    webhooks.borrow_mut().push(id);
                                                }
                                            } else {
                                                webhooks.borrow_mut().retain(|x| x != &id);
                                            }
                                        )
                                    }
                                />
                            }
                        })
                    }
                    </div>
                </div>
            }

            <a
                class="btn btn-primary"
                title="Save"
                onclick={ on_submit }
            >
                <crate::components::Svg icon="check" size=24 />
                { "Save" }
            </a>

            <a
                class="btn btn-secondary"
                title="Cancel"
                onclick={ on_cancel }
            >
                <crate::components::Svg icon="x" size=24 />
                { "Cancel" }
            </a>
        </form>
    }
}
