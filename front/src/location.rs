use gloo::history::History;
use std::collections::HashMap;

pub(crate) struct Location {
    history: gloo::history::BrowserHistory,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            history: gloo::history::BrowserHistory::new(),
        }
    }
}

impl Location {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn path(&self) -> String {
        self.history.location().path().to_string()
    }

    #[must_use]
    pub fn q(&self) -> String {
        self.param("q")
    }

    #[must_use]
    pub fn param(&self, name: &str) -> String {
        self.query().get(name).cloned().unwrap_or_default()
    }

    #[must_use]
    pub fn query(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let query = self.history.location().query_str().to_string();

        for args in query.trim_start_matches('?').split('&') {
            let mut tokens = args.split('=');
            let v = tokens.next().unwrap().to_string();
            let k = tokens.next().unwrap_or_default().to_string();
            map.insert(v, urlencoding::decode(&k).unwrap_or_default().to_string());
        }

        map
    }
}

impl From<Location> for elephantry_extras::Pagination {
    fn from(location: Location) -> Self {
        let query = location.query();

        Self {
            page: query.get("page").and_then(|x| x.parse().ok()).unwrap_or(1),
            limit: query
                .get("limit")
                .and_then(|x| x.parse().ok())
                .unwrap_or(25),
        }
    }
}

pub(crate) fn base_url() -> String {
    "/".to_string()
}

pub(crate) fn set_route(route: &str) {
    let history = gloo::history::BrowserHistory::new();
    history.push(route);
}
