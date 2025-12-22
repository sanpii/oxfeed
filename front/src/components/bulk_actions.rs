pub enum Selection {
    All,
    None,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub active: bool,
    pub disabled: bool,
    pub on_action: yew::Callback<(&'static str, bool)>,
    pub on_select: yew::Callback<Selection>,
    pub on_toggle: yew::Callback<bool>,
    pub on_webhook: yew::Callback<uuid::Uuid>,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let on_toggle = yew_callback::callback!(
        active = props.active,
        on_toggle = props.on_toggle,
        move |_| {
            on_toggle.emit(!active);
        }
    );

    let on_read = yew_callback::callback!(on_action = props.on_action, move |_| {
        on_action.emit(("read", true));
    });
    let on_unread = yew_callback::callback!(on_action = props.on_action, move |_| {
        on_action.emit(("read", false));
    });
    let on_favorite = yew_callback::callback!(on_action = props.on_action, move |_| {
        on_action.emit(("favorite", true));
    });
    let on_unfavorite = yew_callback::callback!(on_action = props.on_action, move |_| {
        on_action.emit(("favorite", false));
    });

    let on_selectall = yew_callback::callback!(on_select = props.on_select, move |_| {
        on_select.emit(Selection::All);
    });
    let on_unselectall = yew_callback::callback!(on_select = props.on_select, move |_| {
        on_select.emit(Selection::None);
    });

    yew::html! {
        <div class="input-group mb-3">
            <fieldset disabled={ props.disabled }>
                <div class="btn-group me-2">
                    <div class="btn btn-outline-secondary">
                        <input type="checkbox" class="form-check-input" checked={ props.active } onclick={ on_toggle } title="Enable bulk actions" />
                    </div>

                    <button type="button" class="btn btn-outline-secondary dropdown-toggle dropdown-toggle-split" data-bs-toggle="dropdown">
                    </button>
                    <ul class="dropdown-menu">
                        <li><a class="dropdown-item" href="#" onclick={ move |_| on_selectall.emit(()) }>{ "All" }</a></li>
                        <li><a class="dropdown-item" href="#" onclick={ move |_| on_unselectall.emit(()) }>{ "None" }</a></li>
                    </ul>
                </div>

                <div class="btn-group me-2">
                    <button type="button" class="btn btn-outline-primary" onclick={ on_read } title="Mark as read">
                        <super::Svg icon="eye" size=24 />
                    </button>
                    <button type="button" class="btn btn-outline-primary" onclick={ on_unread } title="Mark as unread">
                        <super::Svg icon="eye-slash" size=24 />
                    </button>
                </div>

                <div class="btn-group me-2">
                    <button type="button" class="btn btn-outline-warning" onclick={ on_favorite } title="Add to favorites">
                        <super::Svg icon="star-fill" size=24 />
                    </button>
                    <button type="button" class="btn btn-outline-warning" onclick={ on_unfavorite } title="Remove from favorites">
                        <super::Svg icon="star" size=24 />
                    </button>
                </div>

                <Webhooks on_webhook={ props.on_webhook.clone() } />
            </fieldset>
        </div>
    }
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct WebhooksProperties {
    pub on_webhook: yew::Callback<uuid::Uuid>,
}

#[yew::component]
fn Webhooks(props: &WebhooksProperties) -> yew::Html {
    let context = crate::use_context();
    let wait = yew::use_memo(context.clone(), |context| context.fetching);

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

    let on_webhook = yew_callback::callback!(on_webhook = props.on_webhook, move |id| {
        on_webhook.emit(id);
    });

    yew::html! {
        <div class="btn-group">
            <button type="button" class="btn btn-outline-secondary dropdown-toggle" data-bs-toggle="dropdown" title="Execute a webhook">
                if *wait {
                    <super::Svg icon="hourglass-split" size=24 />
                } else {
                    <super::Svg icon="plug" size=24 />
                }
            </button>
            <ul class="dropdown-menu">
            {
                for (*webhooks).clone().into_iter().map(|webhook| {
                    let on_webhook = on_webhook.clone();

                    yew::html! {
                        <li>
                            <a class="dropdown-item" href="#" onclick={ move |_| on_webhook.emit(webhook.id.unwrap()) }>{ &webhook.name }</a>
                        </li>
                    }
                })
            }
            </ul>
        </div>
    }
}
