#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Context {
    pub auth: bool,
    pub alerts: Vec<crate::Alert>,
    pub counts: oxfeed_common::Counts,
    pub unread_items: bool,
    pub need_update: usize,
    pub websocket_error: bool,
    pub route: String,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            auth: !crate::Api::token().is_empty(),
            alerts: Vec::new(),
            counts: oxfeed_common::Counts::default(),
            unread_items: false,
            need_update: 0,
            websocket_error: false,
            route: crate::Location::new().path(),
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
            Logged => context.auth = true,
            AuthRequire => context.auth = false,
            NeedUpdate => context.need_update = context.need_update.overflowing_add(1).0,
            WebsocketError => context.websocket_error = true,
            Route(route) => {
                crate::location::set_route(&route);
                context.route = route;
            }
        }

        context.into()
    }
}
