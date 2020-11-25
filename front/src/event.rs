use std::collections::HashSet;

#[derive(Clone)]
pub(crate) struct Alert {
    pub level: log::Level,
    pub message: String,
}

impl Alert {
    pub fn info(message: &str) -> Self {
        Self::new(log::Level::Info, message)
    }

    fn new(level: log::Level, message: &str) -> Self {
        Self {
            level,
            message: message.to_string(),
        }
    }

    pub fn severity(&self) -> String {
        let severity = match self.level {
            log::Level::Trace => "light",
            log::Level::Debug => "info",
            log::Level::Info => "success",
            log::Level::Warn => "warning",
            log::Level::Error => "danger",
        };

        severity.to_string()
    }
}

#[derive(Clone)]
pub(crate) enum Event {
    Alert(Alert),
    ItemUpdate,
    Search(String),
    SettingUpdate,
}

pub(crate) struct Bus {
    link: yew::agent::AgentLink<Self>,
    subscribers: HashSet<yew::agent::HandlerId>,
}

impl yew::agent::Agent for Bus {
    type Reach = yew::worker::Context<Self>;
    type Message = ();
    type Input = Event;
    type Output = Event;

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
