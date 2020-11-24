use std::collections::HashSet;

#[derive(Clone)]
pub(crate) enum Message {
    ItemUpdate,
    SettingUpdate,
}

pub(crate) struct Bus {
    link: yew::agent::AgentLink<Self>,
    subscribers: HashSet<yew::agent::HandlerId>,
}

impl yew::agent::Agent for Bus {
    type Reach = yew::worker::Context<Self>;
    type Message = ();
    type Input = Message;
    type Output = Message;

    fn create(link: yew::agent::AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _: Self::Message) {
    }

    fn handle_input(&mut self, msg: Self::Input, _: yew::agent::HandlerId) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }

    fn connected(&mut self, id: yew::agent::HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: yew::agent::HandlerId) {
        self.subscribers.remove(&id);
    }
}
