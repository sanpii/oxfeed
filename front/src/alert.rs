#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub(crate) struct Alert {
    pub level: log::Level,
    pub message: String,
}

impl Alert {
    #[must_use]
    pub fn info(message: &str) -> Self {
        Self::new(log::Level::Info, message)
    }

    #[must_use]
    pub fn error(message: &str) -> Self {
        Self::new(log::Level::Error, message)
    }

    fn new(level: log::Level, message: &str) -> Self {
        Self {
            level,
            message: message.to_string(),
        }
    }

    #[must_use]
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

impl From<oxfeed_common::Error> for Alert {
    fn from(error: oxfeed_common::Error) -> Self {
        Self::error(&error.to_string())
    }
}
