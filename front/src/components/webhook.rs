#[derive(Clone)]
pub(crate) enum Message {
    Cancel,
    Delete,
    Deleted,
    Edit,
    Save(oxfeed_common::webhook::Entity),
    Saved(oxfeed_common::webhook::Entity),
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::WebhookDelete(_) => Self::Deleted,
            crate::event::Api::WebhookUpdate(webhook) => Self::Saved(webhook),
            _ => unreachable!(),
        }
    }
}

enum Scene {
    Edit,
    View,
}

#[derive(yew::Properties, Clone)]
pub(crate) struct Properties {
    pub value: oxfeed_common::webhook::Entity,
}

pub(crate) struct Component {
    api: crate::Api<Self>,
    scene: Scene,
    link: yew::ComponentLink<Self>,
    webhook: oxfeed_common::webhook::Entity,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            api: crate::Api::new(link.clone()),
            scene: Scene::View,
            link,
            webhook: props.value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match self.scene {
            Scene::View => match msg {
                Self::Message::Delete => {
                    let message = format!("Would you like delete '{}' webhook?", self.webhook.name);

                    if yew::services::dialog::DialogService::confirm(&message) {
                        self.api.webhooks_delete(&self.webhook.webhook_id.unwrap());
                    }
                }
                Self::Message::Deleted => (),
                Self::Message::Edit => {
                    self.scene = Scene::Edit;
                    return true;
                }
                _ => unreachable!(),
            },
            Scene::Edit => match msg {
                Self::Message::Cancel => {
                    self.scene = Scene::View;
                    return true;
                }
                Self::Message::Save(webhook) => {
                    self.webhook = webhook;
                    self.api
                        .webhooks_update(&self.webhook.webhook_id.unwrap(), &self.webhook);
                    return true;
                }
                Self::Message::Saved(webhook) => {
                    self.webhook = webhook;
                    self.scene = Scene::View;
                    return true;
                }
                _ => unreachable!(),
            },
        }

        false
    }

    fn view(&self) -> yew::Html {
        match &self.scene {
            Scene::Edit => yew::html! {
                <super::form::Webhook
                    webhook=self.webhook.clone()
                    on_cancel=self.link.callback(|_| Self::Message::Cancel)
                    on_submit=self.link.callback(|webhook| Self::Message::Save(webhook))
                />
            },
            Scene::View => {
                let webhook = self.webhook.clone();

                yew::html! {
                    <>
                        <div class="d-inline-flex">
                            { webhook.name }
                            {
                                if let Some(last_error) = webhook.last_error {
                                    yew::html! {
                                        <>
                                            { " · " }
                                            <span class="error">{ last_error }</span>
                                        </>
                                    }
                                }
                                else {
                                    "".into()
                                }
                            }
                        </div>

                        <div class=("btn-group", "float-right")>
                            <button
                                class=("btn", "btn-primary")
                                title="Edit"
                                onclick=self.link.callback(move |_| Self::Message::Edit)
                            >
                                <super::Svg icon="pencil-square" size=16 />
                            </button>
                            <button
                                class=("btn", "btn-danger")
                                title="Delete"
                                onclick=self.link.callback(|_| Message::Delete)
                            >
                                <super::Svg icon="trash" size=16 />
                            </button>
                        </div>
                    </>
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.webhook != props.value;

        self.webhook = props.value;

        should_render
    }
}
