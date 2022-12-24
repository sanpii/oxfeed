pub enum Message {
    Add,
    Cancel,
    Create(oxfeed_common::webhook::Entity),
    Error(String),
    Event(crate::Event),
    NeedUpdate,
    Update(Vec<oxfeed_common::webhook::Entity>),
}

enum Scene {
    Add,
    View,
}

pub struct Component {
    scene: Scene,
    webhooks: Vec<oxfeed_common::webhook::Entity>,
    _producer: Box<dyn yew_agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        use yew_agent::Bridged;

        let callback = {
            let link = ctx.link().clone();
            move |e| link.send_message(Message::Event(e))
        };

        let component = Self {
            scene: Scene::View,
            webhooks: Vec::new(),
            _producer: crate::event::Bus::bridge(std::rc::Rc::new(callback)),
        };

        ctx.link().send_message(Message::NeedUpdate);

        component
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = true;

        match &self.scene {
            Scene::View => match msg {
                Message::Add => self.scene = Scene::Add,
                Message::Update(ref webhooks) => self.webhooks = webhooks.clone(),
                _ => (),
            },
            Scene::Add => match msg {
                Message::Cancel => self.scene = Scene::View,
                Message::Create(ref webhook) => crate::api!(
                    ctx.link(),
                    webhooks_create(webhook) -> |_| Message::Event(crate::Event::WebhookUpdate)
                ),
                _ => (),
            },
        }

        if let Message::Event(ref event) = msg {
            if matches!(event, crate::Event::WebhookUpdate) {
                ctx.link().send_message(Message::NeedUpdate);
            }
        } else if matches!(msg, Message::NeedUpdate) {
            self.scene = Scene::View;

            crate::api!(
                ctx.link(),
                webhooks_all() -> Message::Update
            );

            should_render = false;
        }

        should_render
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        use crate::Render;

        yew::html! {
            <>
            {
                if matches!(self.scene, Scene::View) {
                    yew::html! {
                        <a
                            class={ yew::classes!("btn", "btn-primary") }
                            title="Add"
                            onclick={ ctx.link().callback(|_| Message::Add) }
                        >
                            <crate::components::Svg icon="plus" size=24 />
                            { "Add" }
                        </a>
                    }
                } else {
                    "".into()
                }
            }
            <ul class="list-group">
            {
                if matches!(self.scene, Scene::Add) {
                    yew::html! {
                        <li class="list-group-item">
                            <crate::components::form::Webhook
                                webhook={ oxfeed_common::webhook::Entity::default() }
                                on_cancel={ ctx.link().callback(|_| Message::Cancel) }
                                on_submit={ ctx.link().callback(Message::Create) }
                            />
                        </li>
                    }
                } else {
                    "".into()
                }
            }
            {
                for self.webhooks.iter().map(|webhook| {
                    yew::html!{
                        <li class="list-group-item">{ webhook.render() }</li>
                    }
                })
            }
            </ul>
            </>
        }
    }

    crate::change!();
}
