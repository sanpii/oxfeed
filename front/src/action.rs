#[derive(Debug)]
pub(crate) enum Action {
    AuthRequire,
    AddAlert(crate::Alert),
    Logged,
    RemoveAlert(usize),
    WebsocketError,
    NeedUpdate,
    Route(String),
}

impl From<crate::Alert> for Action {
    fn from(value: crate::Alert) -> Self {
        Self::AddAlert(value)
    }
}

impl From<oxfeed_common::Error> for Action {
    fn from(value: oxfeed_common::Error) -> Self {
        Self::AddAlert(value.into())
    }
}
