use std::collections::HashSet;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub enum Event {
    Alert(Alert),
    AuthRequire,
    ItemUpdate,
    Logged,
    SettingUpdate,
    SourceUpdate,
    Redirect(String),
    Redirected(String),
    WebhookUpdate,
}

impl From<&oxfeed_common::Error> for Event {
    fn from(error: &oxfeed_common::Error) -> Self {
        let alert = crate::event::Alert::error(&error.to_string());

        Self::Alert(alert)
    }
}

impl From<oxfeed_common::Error> for Event {
    fn from(error: oxfeed_common::Error) -> Self {
        error.into()
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Alert {
    pub level: log::Level,
    pub message: String,
}

impl Alert {
    pub fn info(message: &str) -> Self {
        Self::new(log::Level::Info, message)
    }

    pub fn error(message: &str) -> Self {
        Self::new(log::Level::Error, message)
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

pub struct Bus {
    link: yew_agent::WorkerLink<Self>,
    subscribers: HashSet<yew_agent::HandlerId>,
}

impl yew_agent::Worker for Bus {
    type Reach = yew_agent::Public<Self>;
    type Message = ();
    type Input = Event;
    type Output = Event;

    fn create(link: yew_agent::WorkerLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _: yew_agent::HandlerId) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }

    fn connected(&mut self, id: yew_agent::HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: yew_agent::HandlerId) {
        self.subscribers.remove(&id);
    }

    fn name_of_resource() -> &'static str {
        "event_bus.js"
    }
}
