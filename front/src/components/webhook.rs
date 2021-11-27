pub(crate) enum Message {
    Cancel,
    Delete,
    Deleted,
    Edit,
    Error(String),
    Save(oxfeed_common::webhook::Entity),
    Saved(oxfeed_common::webhook::Entity),
}

enum Scene {
    Edit,
    View,
}

#[derive(yew::Properties, Clone, PartialEq)]
pub(crate) struct Properties {
    pub value: oxfeed_common::webhook::Entity,
}

pub(crate) struct Component {
    scene: Scene,
    event_bus: yew_agent::Dispatcher<crate::event::Bus>,
    value: oxfeed_common::webhook::Entity,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        use yew_agent::Dispatched;

        Self {
            scene: Scene::View,
            event_bus: crate::event::Bus::dispatcher(),
            value: ctx.props().value.clone(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;

        match self.scene {
            Scene::View => match msg {
                Message::Delete => {
                    let message = format!("Would you like delete '{}' webhook?", self.value.name);

                    if gloo::dialogs::confirm(&message) {
                        let id = &self.value.id.unwrap();

                        crate::api!(
                            ctx.link(),
                            webhooks_delete(id) -> |_| Message::Deleted
                        );
                    }
                }
                Message::Deleted => self.event_bus.send(crate::Event::WebhookUpdate),
                Message::Edit => {
                    self.scene = Scene::Edit;
                    should_render = true;
                }
                _ => unreachable!(),
            },
            Scene::Edit => match msg {
                Message::Cancel => {
                    self.scene = Scene::View;
                    should_render = true;
                }
                Message::Save(webhook) => {
                    let id = &webhook.id.unwrap();
                    self.value = webhook.clone();

                    crate::api!(
                        ctx.link(),
                        webhooks_update(id, webhook) -> Message::Saved
                    );

                    should_render = true;
                }
                Message::Saved(webhook) => {
                    self.value = webhook;
                    self.scene = Scene::View;
                    self.event_bus.send(crate::Event::WebhookUpdate);
                    should_render = true;
                }
                _ => unreachable!(),
            },
        }

        should_render
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        match &self.scene {
            Scene::Edit => yew::html! {
                <super::form::Webhook
                    webhook={ self.value.clone() }
                    on_cancel={ ctx.link().callback(|_| Message::Cancel) }
                    on_submit={ ctx.link().callback(Message::Save) }
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
                                        <super::Error text={ last_error } />
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
                                onclick={ ctx.link().callback(move |_| Message::Edit) }
                            >
                                <super::Svg icon="pencil-square" size=16 />
                            </button>
                            <button
                                class={ yew::classes!("btn", "btn-danger") }
                                title="Delete"
                                onclick={ ctx.link().callback(|_| Message::Delete) }
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
