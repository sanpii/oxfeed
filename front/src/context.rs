#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Context {
    pub auth: bool,
    pub alerts: Vec<crate::Alert>,
    pub counts: oxfeed::Counts,
    pub fetching: bool,
    pub need_update: usize,
    pub websocket_error: bool,
    pub route: String,
    pub theme: Theme,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            auth: !crate::Api::token().is_empty(),
            alerts: Vec::new(),
            counts: oxfeed::Counts::default(),
            fetching: false,
            need_update: 0,
            websocket_error: false,
            route: crate::Location::new().path(),
            theme: Theme::default(),
        }
    }
}

impl yew::Reducible for Context {
    type Action = crate::Action;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        use crate::Action::*;

        let mut context = (*self).clone();

        match action {
            AddAlert(alert) => context.alerts.push(alert),
            RemoveAlert(idx) => {
                context.alerts.remove(idx);
            }
            Fetch => context.fetching = true,
            Fetched => context.fetching = false,
            Logged => context.auth = true,
            AuthRequire => {
                context.auth = false;
                context.websocket_error = false;
            }
            NeedUpdate => context.need_update = context.need_update.overflowing_add(1).0,
            WebsocketError => context.websocket_error = true,
            Route(route) => {
                crate::location::set_route(&route);
                context.route = route;
            }
            Theme(theme) => context.theme = theme,
        }

        context.into()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) enum Theme {
    Auto,
    Dark,
    Light,
}

impl Theme {
    pub const fn icon(&self) -> &'static str {
        match self {
            Self::Auto => "circle-half",
            Self::Dark => "moon-stars-fill",
            Self::Light => "sun-fill",
        }
    }

    pub const fn all() -> [Self; 3] {
        [Theme::Light, Theme::Dark, Theme::Auto]
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Auto => "Auto",
            Self::Dark => "Dark",
            Self::Light => "Light",
        };

        f.write_str(s)
    }
}

impl Default for Theme {
    fn default() -> Self {
        use gloo::storage::Storage as _;

        gloo::storage::LocalStorage::get("theme").unwrap_or(Self::Auto)
    }
}
