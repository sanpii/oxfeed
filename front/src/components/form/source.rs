pub(crate) enum Message {
    Cancel,
    Submit,
    ToggleActive(bool),
    ToggleWebhook(uuid::Uuid, bool),
    UpdateTags(Vec<String>),
    UpdateTitle(String),
    UpdateUrl(String),
    Webhooks(Vec<oxfeed_common::webhook::Entity>),
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::Webhooks(webhooks) => Self::Webhooks(webhooks),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub source: oxfeed_common::source::Entity,
    pub on_cancel: yew::Callback<()>,
    pub on_submit: yew::Callback<oxfeed_common::source::Entity>,
}

pub(crate) struct Component {
    api: crate::Api<Self>,
    link: yew::ComponentLink<Self>,
    props: Properties,
    webhooks: Vec<oxfeed_common::webhook::Entity>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let mut component = Self {
            api: crate::Api::new(link.clone()),
            link,
            props,
            webhooks: Vec::new(),
        };

        component.api.webhooks_all();

        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Cancel => self.props.on_cancel.emit(()),
            Self::Message::Submit => self.props.on_submit.emit(self.props.source.clone()),
            Self::Message::ToggleActive(active) => self.props.source.active = active,
            Self::Message::ToggleWebhook(id, active) => {
                if active {
                    if !self.props.source.webhooks.contains(&id) {
                        self.props.source.webhooks.push(id)
                    }
                } else {
                    self.props.source.webhooks.retain(|x| x != &id);
                }
            }
            Self::Message::UpdateTags(tags) => self.props.source.tags = tags,
            Self::Message::UpdateTitle(title) => self.props.source.title = title,
            Self::Message::UpdateUrl(url) => self.props.source.url = url,
            Self::Message::Webhooks(webhooks) => self.webhooks = webhooks,
        }

        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <form>
                <div class="row mb-3">
                    <label class="col-sm-1 col-form-label" for="title">{ "Title" }</label>
                    <div class="col-sm-11">
                        <input
                            class="form-control"
                            name="title"
                            value=self.props.source.title.clone()
                            oninput=self.link.callback(|e: yew::InputData| Self::Message::UpdateTitle(e.value))
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
                            value=self.props.source.url.clone()
                            oninput=self.link.callback(|e: yew::InputData| Self::Message::UpdateUrl(e.value))
                        />
                    </div>
                </div>

                <div class="row mb-3">
                    <label class="col-sm-1 col-form-label" for="tags">{ "Tags" }</label>
                    <div class="col-sm-11">
                        <super::Tags
                            values=self.props.source.tags.clone()
                            on_change=self.link.callback(Self::Message::UpdateTags)
                        />
                    </div>
                </div>

                <div class="row mb-3">
                    <div class="col-sm-11 offset-sm-1">
                        <crate::components::Switch
                            id="active"
                            active=self.props.source.active
                            on_toggle=self.link.callback(Self::Message::ToggleActive)
                            label="active"
                        />
                    </div>
                </div>

                {
                    if !self.webhooks.is_empty() {
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
                                                id=id.to_string()
                                                label=webhook.name.clone()
                                                active=active
                                                on_toggle=self.link.callback(move |active| Self::Message::ToggleWebhook(id, active))
                                            />
                                        }
                                    })
                                }
                                </div>
                            </div>
                        }
                    } else {
                        "".into()
                    }
                }

                <a
                    class=yew::classes!("btn", "btn-primary")
                    title="Save"
                    onclick=self.link.callback(|_| Self::Message::Submit)
                >
                    <crate::components::Svg icon="check" size=24 />
                    { "Save" }
                </a>

                <a
                    class=yew::classes!("btn", "btn-secondary")
                    title="Cancel"
                    onclick=self.link.callback(|_| Self::Message::Cancel)
                >
                    <crate::components::Svg icon="x" size=24 />
                    { "Cancel" }
                </a>
            </form>
        }
    }

    crate::change!(props);
}
