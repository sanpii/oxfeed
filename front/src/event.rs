use std::collections::HashSet;

#[derive(Clone)]
pub(crate) enum Event {
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

#[derive(Clone)]
pub(crate) struct Alert {
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

pub(crate) struct Bus {
    link: yew_agent::AgentLink<Self>,
    subscribers: HashSet<yew_agent::HandlerId>,
}

impl yew_agent::Agent for Bus {
    type Reach = yew_agent::Context<Self>;
    type Message = ();
    type Input = Event;
    type Output = Event;

    fn create(link: yew_agent::AgentLink<Self>) -> Self {
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
}
