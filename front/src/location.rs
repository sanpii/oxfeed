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

    pub fn query(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let query = self.router.get_query();

        for args in query.trim_start_matches('?').split('&') {
            let mut tokens = args.split('=');
            map.insert(
                tokens.next().unwrap().to_string(),
                tokens.next().unwrap_or_default().to_string(),
            );
        }

        map
    }

    pub fn redirect(&self, url: &str) {
        let location = yew::utils::document().location().unwrap();
        location.set_href(url).ok();
    }

    pub fn reload(&self) {
        let location = yew::utils::document().location().unwrap();
        location.reload().ok();
    }
}

impl From<Location> for oxfeed_common::Pagination {
    fn from(location: Location) -> Self {
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

pub(crate) fn base_url() -> String {
    let location = Location::new();

    location.path()
}
