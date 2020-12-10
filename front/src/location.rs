use std::collections::HashMap;

pub(crate) struct Location {
    router: yew_router::service::RouteService<()>,
}

impl Location {
    pub fn new() -> Self {
        let router = yew_router::service::RouteService::<()>::new();

        Self { router }
    }

    pub fn path(&self) -> String {
        self.router.get_path()
    }

    pub fn set_path(&mut self, path: &str) {
        use yew::agent::Dispatched;

        let route = yew_router::route::Route {
            route: path.to_string(),
            state: (),
        };

        self.router.set_route(&route.route, ());

        let mut dispatcher = yew_router::agent::RouteAgentDispatcher::<()>::new();
        dispatcher.send(yew_router::agent::RouteRequest::ChangeRoute(route));

        let mut event_bus = crate::event::Bus::dispatcher();
        event_bus.send(crate::event::Event::Redirected(path.to_string()));
    }

    pub fn q(&self) -> String {
        self.query().get("q").cloned().unwrap_or_default()
    }

    pub fn query(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let query = self.router.get_query();

        for args in query.trim_start_matches('?').split('&') {
            let mut tokens = args.split('=');
            let v = tokens.next().unwrap().to_string();
            let k = tokens.next().unwrap_or_default().to_string();
            map.insert(v, urlencoding::decode(&k).unwrap_or_default());
        }

        map
    }

    pub fn reload(&self) {
        let location = yew::utils::document().location().unwrap();
        location.reload().ok();
    }
}

impl From<&Location> for oxfeed_common::Pagination {
    fn from(location: &Location) -> Self {
        let query = location.query();

        Self {
            page: query
                .get("page")
                .map(|x| x.parse().ok())
                .flatten()
                .unwrap_or(1),
            limit: query
                .get("limit")
                .map(|x| x.parse().ok())
                .flatten()
                .unwrap_or(25),
        }
    }
}

impl std::ops::Deref for Location {
    type Target = yew_router::service::RouteService<()>;

    fn deref(&self) -> &Self::Target {
        &self.router
    }
}

impl std::ops::DerefMut for Location {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.router
    }
}

pub(crate) fn base_url() -> String {
    let location = Location::new();

    location.path()
}
