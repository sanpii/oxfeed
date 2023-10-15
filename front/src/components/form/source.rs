pub enum Message {
    Cancel,
    Error(String),
    Submit,
    ToggleActive(bool),
    ToggleWebhook(uuid::Uuid, bool),
    UpdateTags(Vec<String>),
    UpdateTitle(String),
    UpdateUrl(String),
    Webhooks(Vec<oxfeed_common::webhook::Entity>),
}

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    pub source: oxfeed_common::source::Entity,
    pub on_cancel: yew::Callback<()>,
    pub on_submit: yew::Callback<oxfeed_common::source::Entity>,
}

pub struct Component {
    props: Properties,
    webhooks: Vec<oxfeed_common::webhook::Entity>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let component = Self {
            props: ctx.props().clone(),
            webhooks: Vec::new(),
        };

        crate::api!(
            ctx.link(),
            webhooks_all() -> Message::Webhooks
        );

        component
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Cancel => self.props.on_cancel.emit(()),
            Message::Error(err) => crate::send_error(ctx, &err),
            Message::Submit => self.props.on_submit.emit(self.props.source.clone()),
            Message::ToggleActive(active) => self.props.source.active = active,
            Message::ToggleWebhook(id, active) => {
                if active {
                    if !self.props.source.webhooks.contains(&id) {
                        self.props.source.webhooks.push(id);
                    }
                } else {
                    self.props.source.webhooks.retain(|x| x != &id);
                }
            }
            Message::UpdateTags(tags) => self.props.source.tags = tags,
            Message::UpdateTitle(title) => self.props.source.title = title,
            Message::UpdateUrl(url) => self.props.source.url = url,
            Message::Webhooks(webhooks) => self.webhooks = webhooks,
        }

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        use yew::TargetCast;

        yew::html! {
            <form>
                <div class="row mb-3">
                    <label class="col-sm-1 col-form-label" for="title">{ "Title" }</label>
                    <div class="col-sm-11">
                        <input
                            class="form-control"
                            name="title"
                            value={ self.props.source.title.clone() }
                            oninput={ ctx.link().callback(|e: yew::InputEvent| {
                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                Message::UpdateTitle(input.value())
                            }) }
                        />
                        <small class="form-text text-muted">{ "Leave empty to use the feed title." }</small>
                    </div>
                </div>

                <div class="row mb-3">
                    <label class="col-sm-1 col-form-label" for="url">{ "Feed URL" }</label>
                    <div class="col-sm-11">
                        <input
                            class="form-control"
                            name="url"
                            required=true
                            value={ self.props.source.url.clone() }
                            oninput={ ctx.link().callback(|e: yew::InputEvent| {
                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                Message::UpdateUrl(input.value())
                            }) }
                        />
                    </div>
                </div>

                <div class="row mb-3">
                    <label class="col-sm-1 col-form-label" for="tags">{ "Tags" }</label>
                    <div class="col-sm-11">
                        <super::Tags
                            values={ self.props.source.tags.clone() }
                            on_change={ ctx.link().callback(Message::UpdateTags) }
                        />
                    </div>
                </div>

                <div class="row mb-3">
                    <div class="col-sm-11 offset-sm-1">
                        <crate::components::Switch
                            id="active"
                            active={ self.props.source.active }
                            on_toggle={ ctx.link().callback(Message::ToggleActive) }
                            label="active"
                        />
                    </div>
                </div>

                {
                    if self.webhooks.is_empty() {
                        "".into()
                    } else {
                        yew::html! {
                            <div class="row mb-3">
                                <label class="col-sm-1 col-form-label" for="webhooks">{ "Webhooks" }</label>
                                <div class="col-sm-11">
                                {
                                    for self.webhooks.iter().map(move |webhook| {
                                        let id = webhook.id.unwrap_or_default();
                                        let active = self.props.source.webhooks.contains(&id);

                                        yew::html! {
                                            <crate::components::Switch
                                                id={ id.to_string() }
                                                label={ webhook.name.clone() }
                                                active={ active }
                                                on_toggle={ ctx.link().callback(move |active| Message::ToggleWebhook(id, active)) }
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
                    onclick={ ctx.link().callback(|_| Message::Submit) }
                >
                    <crate::components::Svg icon="check" size=24 />
                    { "Save" }
                </a>

                <a
                    class={ yew::classes!("btn", "btn-secondary") }
                    title="Cancel"
                    onclick={ ctx.link().callback(|_| Message::Cancel) }
                >
                    <crate::components::Svg icon="x" size=24 />
                    { "Cancel" }
                </a>
            </form>
        }
    }

    crate::change!(props);
}
