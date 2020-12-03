mod auth;
mod items;
mod opml;
mod sources;
mod user;

use yew::agent::Dispatched;

#[derive(Clone, Copy)]
enum Kind {
    AuthLogin(bool),
    AuthLogout,
    Counts,
    Items,
    ItemsRead,
    ItemContent,
    ItemPatch,
    OpmlImport,
    SearchItems,
    SearchSources,
    SearchTags,
    Sources,
    SourceCreate,
    SourceDelete,
    SourceUpdate,
    UserCreate,
}

pub(crate) struct Api<C>
where
    C: yew::Component,
    <C as yew::Component>::Message: std::convert::TryFrom<crate::event::Api>,
{
    link: yew::ComponentLink<C>,
    tasks: Vec<yew::services::fetch::FetchTask>,
}

impl<C> Api<C>
where
    C: yew::Component,
    <C as yew::Component>::Message: From<crate::event::Api>,
{
    pub fn new(link: yew::ComponentLink<C>) -> Self {
        Self {
            link,
            tasks: Vec::new(),
        }
    }

    pub fn counts(&mut self) {
        self.fetch(
            Kind::Counts,
            http::Method::GET,
            "/counts",
            yew::format::Nothing,
        )
    }

    pub fn search(&mut self, what: &str, query: &str, pagination: &crate::Pagination) {
        let q = urlencoding::encode(query);
        let url = format!(
            "/search/{}?q={}&page={}&limit={}",
            what, q, pagination.page, pagination.limit
        );

        let kind = match what {
            "all" | "unread" | "favorites" => Kind::SearchItems,
            "sources" => Kind::SearchSources,
            "tags" => Kind::SearchTags,
            _ => {
                log::error!("Unknow '{}' search type", what);
                unreachable!();
            }
        };

        self.fetch(kind, http::Method::GET, &url, yew::format::Nothing)
    }

    pub fn token() -> String {
        wasm_cookies::get("token")
            .unwrap_or_else(|| Ok(String::new()))
            .unwrap_or_default()
    }

    fn set_token(token: &str, remember_me: bool) {
        let expires = std::time::Duration::from_secs(365 * 24 * 60 * 60);
        let mut options = wasm_cookies::CookieOptions::default().expires_after(expires);

        if !remember_me {
            options.expires = None;
        }

        wasm_cookies::set("token", token, &options);
    }

    fn clear_token() {
        wasm_cookies::delete("token");
    }

    fn fetch<B>(&mut self, kind: Kind, method: http::Method, url: &str, body: B)
    where
        B: Into<Result<String, anyhow::Error>>,
    {
        let request = match yew::services::fetch::Request::builder()
            .method(method)
            .uri(&format!("{}{}", env!("API_URL"), url))
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", Self::token()))
            .body(body)
        {
            Ok(request) => request,
            Err(err) => {
                Self::error(err.into());
                return;
            }
        };

        let callback = self.link.batch_callback(
            move |response: yew::services::fetch::Response<yew::format::Text>| {
                use std::convert::TryFrom;

                let event = match Self::on_response(kind, response) {
                    Ok(event) => event,
                    Err(err) => {
                        Self::error(err);
                        return Vec::new();
                    }
                };

                let mut event_bus = crate::event::Bus::dispatcher();
                event_bus.send(crate::event::Event::Api(event.clone()));

                match <C as yew::Component>::Message::try_from(event) {
                    Ok(message) => vec![message],
                    Err(_) => {
                        log::error!("fetch error");
                        Vec::new()
                    }
                }
            },
        );

        match yew::services::FetchService::fetch(request, callback) {
            Ok(task) => self.tasks.push(task),
            Err(err) => {
                Self::error(err.into());
            }
        };
    }

    fn on_response(
        kind: Kind,
        response: yew::services::fetch::Response<yew::format::Text>,
    ) -> crate::Result<crate::event::Api> {
        if response.status() == http::status::StatusCode::UNAUTHORIZED {
            let mut event_bus = crate::event::Bus::dispatcher();

            event_bus.send(crate::event::Event::AuthRequire);
            return Err(crate::Error::Auth);
        }

        let data = response.into_body()?;

        let api_event = match kind {
            Kind::AuthLogin(remember_me) => {
                Self::set_token(&data, remember_me);
                crate::event::Api::Auth
            }
            Kind::AuthLogout => {
                Self::clear_token();
                crate::event::Api::Auth
            }
            Kind::Counts => {
                let counts = serde_json::from_str(&data)?;
                crate::event::Api::Counts(counts)
            }
            Kind::Items => {
                let items = serde_json::from_str(&data)?;
                crate::event::Api::Items(items)
            }
            Kind::ItemsRead => crate::event::Api::ItemsRead,
            Kind::ItemContent => crate::event::Api::ItemContent(data),
            Kind::ItemPatch => crate::event::Api::ItemPatch,
            Kind::OpmlImport => crate::event::Api::OpmlImport,
            Kind::SearchItems => {
                let items = serde_json::from_str(&data)?;
                crate::event::Api::SearchItems(items)
            }
            Kind::SearchSources => {
                let sources = serde_json::from_str(&data)?;
                crate::event::Api::SearchSources(sources)
            }
            Kind::SearchTags => {
                let tags = serde_json::from_str(&data)?;
                crate::event::Api::SearchTags(tags)
            }
            Kind::Sources => {
                let sources = serde_json::from_str(&data)?;
                crate::event::Api::Sources(sources)
            }
            Kind::SourceCreate => {
                let source = serde_json::from_str(&data)?;
                crate::event::Api::SourceCreate(source)
            }
            Kind::SourceDelete => {
                let source = serde_json::from_str(&data)?;
                crate::event::Api::SourceDelete(source)
            }
            Kind::SourceUpdate => {
                let source = serde_json::from_str(&data)?;
                crate::event::Api::SourceUpdate(source)
            }
            Kind::UserCreate => {
                let user = serde_json::from_str(&data)?;
                crate::event::Api::UserCreate(user)
            }
        };

        let event = match kind {
            Kind::SourceCreate | Kind::SourceDelete | Kind::SourceUpdate => Some(crate::event::Event::SourceUpdate),
            Kind::ItemPatch => Some(crate::event::Event::ItemUpdate),
            _ => None,
        };

        if let Some(event) = event {
            let mut event_bus = crate::event::Bus::dispatcher();
            event_bus.send(event.clone());
        }

        Ok(api_event)
    }

    fn error(error: crate::Error) {
        let mut event_bus = crate::event::Bus::dispatcher();
        event_bus.send(error.into());
    }
}
