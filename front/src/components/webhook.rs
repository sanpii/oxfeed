pub(crate) enum Message {
    Cancel,
    Delete,
    Deleted,
    Edit,
    Error(oxfeed_common::Error),
    Save(oxfeed_common::webhook::Entity),
    Saved(oxfeed_common::webhook::Entity),
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
    scene: Scene,
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    link: yew::ComponentLink<Self>,
    value: oxfeed_common::webhook::Entity,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Dispatched;

        Self {
            scene: Scene::View,
            event_bus: crate::event::Bus::dispatcher(),
            link,
            value: props.value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        use yewtil::future::LinkFuture;

        match self.scene {
            Scene::View => match msg {
                Self::Message::Delete => {
                    let message = format!("Would you like delete '{}' webhook?", self.value.name);

                    if yew::services::dialog::DialogService::confirm(&message) {
                        let id = self.value.id;

                        self.link.send_future(async move {
                            crate::Api::webhooks_delete(&id.unwrap())
                                .await
                                .map_or_else(Self::Message::Error, |_| Self::Message::Deleted)
                        });
                    }
                }
                Self::Message::Deleted => self.event_bus.send(crate::event::Event::WebhookUpdate),
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
                    self.value = webhook.clone();

                    self.link.send_future(async move {
                        crate::Api::webhooks_update(&webhook.id.unwrap(), &webhook)
                            .await
                            .map_or_else(Self::Message::Error, Self::Message::Saved)
                    });

                    return true;
                }
                Self::Message::Saved(webhook) => {
                    self.value = webhook;
                    self.scene = Scene::View;
                    self.event_bus.send(crate::event::Event::WebhookUpdate);
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
                    webhook=self.value.clone()
                    on_cancel=self.link.callback(|_| Self::Message::Cancel)
                    on_submit=self.link.callback(Self::Message::Save)
                />
            },
            Scene::View => {
                let webhook = self.value.clone();

                yew::html! {
                    <>
                        <div class="d-inline-flex">
                            { webhook.name }
                            {
                                if let Some(last_error) = webhook.last_error {
                                    yew::html! {
                                        <super::Error text=last_error />
                                    }
                                }
                                else {
                                    "".into()
                                }
                            }
                        </div>

                        <div class=yew::classes!("btn-group", "float-end")>
                            <button
                                class=yew::classes!("btn", "btn-primary")
                                title="Edit"
                                onclick=self.link.callback(move |_| Self::Message::Edit)
                            >
                                <super::Svg icon="pencil-square" size=16 />
                            </button>
                            <button
                                class=yew::classes!("btn", "btn-danger")
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

    crate::change!(value);
}
