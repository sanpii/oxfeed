pub(crate) enum Message {
    Add,
    Cancel,
    Create(oxfeed_common::webhook::Entity),
    Event(crate::event::Event),
    NeedUpdate,
    Update(Vec<oxfeed_common::webhook::Entity>),
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::Webhooks(webhooks) => Self::Update(webhooks),
            crate::event::Api::WebhookCreate(_) => Self::NeedUpdate,
            crate::event::Api::WebhookDelete(_) => Self::NeedUpdate,
            crate::event::Api::WebhookUpdate(_) => Self::NeedUpdate,
            _ => unreachable!(),
        }
    }
}

enum Scene {
    Add,
    View,
}

pub(crate) struct Component {
    api: crate::Api<Self>,
    link: yew::ComponentLink<Self>,
    scene: Scene,
    webhooks: Vec<oxfeed_common::webhook::Entity>,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Bridged;

        let callback = link.callback(Self::Message::Event);

        let component = Self {
            api: crate::Api::new(link.clone()),
            link,
            scene: Scene::View,
            webhooks: Vec::new(),
            _producer: crate::event::Bus::bridge(callback),
        };

        component.link.send_message(Self::Message::NeedUpdate);

        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match &self.scene {
            Scene::View => match msg {
                Self::Message::Add => self.scene = Scene::Add,
                Self::Message::Update(ref webhooks) => self.webhooks = webhooks.clone(),
                _ => (),
            },
            Scene::Add => match msg {
                Self::Message::Cancel => self.scene = Scene::View,
                Self::Message::Create(ref webhook) => self.api.webhooks_create(webhook),
                _ => (),
            },
        }

        if let Self::Message::Event(ref event) = msg {
            if matches!(event, crate::event::Event::WebhookUpdate) {
                self.link.send_message(Self::Message::NeedUpdate);
            }
        } else if matches!(msg, Self::Message::NeedUpdate) {
            self.scene = Scene::View;
            self.api.webhooks_all();
            return false;
        }

        true
    }

    fn view(&self) -> yew::Html {
        use crate::Render;

        yew::html! {
            <>
            {
                if matches!(self.scene, Scene::View) {
                    yew::html! {
                        <a
                            class=yew::classes!("btn", "btn-primary")
                            title="Add"
                            onclick=self.link.callback(|_| Message::Add)
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
                                webhook=oxfeed_common::webhook::Entity::default()
                                on_cancel=self.link.callback(|_| Self::Message::Cancel)
                                on_submit=self.link.callback(Self::Message::Create)
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
