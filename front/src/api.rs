use reqwest::Method;
use yew::agent::Dispatched;

pub(crate) struct Api;

impl Api {
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

    pub async fn counts() -> oxfeed_common::Result<oxfeed_common::Counts> {
        Self::fetch(Method::GET, "/counts", yew::format::Nothing).await
    }

    pub async fn auth_login(
        email: &str,
        password: &str,
        remember_me: &bool,
    ) -> oxfeed_common::Result {
        use hmac::NewMac;
        use jwt::SignWithKey;

        let key: hmac::Hmac<sha2::Sha256> =
            hmac::Hmac::new_from_slice(env!("SECRET").as_bytes()).unwrap();
        let mut claims = std::collections::BTreeMap::new();
        claims.insert("email", email);
        claims.insert("password", password);

        let token = claims.sign_with_key(&key).unwrap();

        let data: String =
            Self::fetch(Method::POST, "/auth/login", yew::format::Json(&token)).await?;

        Self::set_token(&data, *remember_me);

        Ok(())
    }

    pub async fn auth_logout() -> oxfeed_common::Result {
        Self::fetch(Method::POST, "/auth/logout", yew::format::Nothing).await?;

        Self::clear_token();

        Ok(())
    }

    pub async fn items_all(
        kind: &str,
        pagination: &oxfeed_common::Pagination,
    ) -> oxfeed_common::Result<crate::Pager<oxfeed_common::item::Item>> {
        let kind = if kind == "all" {
            String::new()
        } else {
            kind.to_string()
        };

        let url = format!("/items/{}?{}", kind, pagination.to_query());

        Self::fetch(Method::GET, &url, yew::format::Nothing).await
    }

    pub async fn items_content(id: &uuid::Uuid) -> oxfeed_common::Result<String> {
        let url = format!("/items/{}/content", id);

        Self::fetch(Method::GET, &url, yew::format::Nothing).await
    }

    pub async fn items_read() -> oxfeed_common::Result {
        Self::fetch(Method::POST, "/items/read", yew::format::Nothing).await
    }

    pub async fn items_tag(id: &uuid::Uuid, key: &str, value: &bool) -> oxfeed_common::Result {
        let url = format!("/items/{}", id);

        let json = serde_json::json!({
            key: *value,
        });

        Self::fetch(Method::PATCH, &url, yew::format::Json(&json)).await
    }

    pub async fn items_search(
        what: &str,
        filter: &crate::Filter,
        pagination: &oxfeed_common::Pagination,
    ) -> oxfeed_common::Result<crate::Pager<oxfeed_common::item::Item>> {
        let url = format!(
            "/search/{}?{}&{}",
            what,
            filter.to_url_param(),
            pagination.to_query()
        );

        Self::fetch(Method::GET, &url, yew::format::Nothing).await
    }

    pub async fn opml_import(
        opml: Result<std::string::String, anyhow::Error>,
    ) -> oxfeed_common::Result {
        Self::fetch(Method::POST, "/opml", opml).await
    }

    pub async fn sources_all(
        pagination: &oxfeed_common::Pagination,
    ) -> oxfeed_common::Result<crate::Pager<oxfeed_common::source::Entity>> {
        let url = format!("/sources?{}", pagination.to_query());

        Self::fetch(Method::GET, &url, yew::format::Nothing).await
    }

    pub async fn sources_create(
        source: &oxfeed_common::source::Entity,
    ) -> oxfeed_common::Result<oxfeed_common::source::Entity> {
        Self::fetch(Method::POST, "/sources", source).await
    }

    pub async fn sources_update(
        id: &uuid::Uuid,
        source: &oxfeed_common::source::Entity,
    ) -> oxfeed_common::Result<oxfeed_common::source::Entity> {
        Self::fetch(Method::PUT, &format!("/sources/{}", id), source).await
    }

    pub async fn sources_delete(
        id: &uuid::Uuid,
    ) -> oxfeed_common::Result<oxfeed_common::source::Entity> {
        Self::fetch(
            Method::DELETE,
            &format!("/sources/{}", id),
            yew::format::Nothing,
        )
        .await
    }

    pub async fn sources_search(
        filter: &crate::Filter,
        pagination: &oxfeed_common::Pagination,
    ) -> oxfeed_common::Result<crate::Pager<oxfeed_common::source::Entity>> {
        let url = format!(
            "/search/sources?{}&{}",
            filter.to_url_param(),
            pagination.to_query()
        );

        Self::fetch(Method::GET, &url, yew::format::Nothing).await
    }

    pub async fn tags_all(
        pagination: &oxfeed_common::Pagination,
    ) -> oxfeed_common::Result<Vec<oxfeed_common::Tag>> {
        let url = format!("/tags?{}", pagination.to_query());

        Self::fetch(Method::GET, &url, yew::format::Nothing).await
    }

    pub async fn tags_search(
        filter: &crate::Filter,
        pagination: &oxfeed_common::Pagination,
    ) -> oxfeed_common::Result<Vec<String>> {
        let url = format!(
            "/search/tags?{}&{}",
            filter.to_url_param(),
            pagination.to_query()
        );

        Self::fetch(Method::GET, &url, yew::format::Nothing).await
    }

    pub async fn user_create(user: &oxfeed_common::new_user::Entity) -> oxfeed_common::Result {
        Self::fetch(Method::POST, "/users", user).await
    }

    pub async fn webhooks_all() -> oxfeed_common::Result<Vec<oxfeed_common::webhook::Entity>> {
        Self::fetch(Method::GET, "/webhooks", yew::format::Nothing).await
    }

    pub async fn webhooks_create(
        webhook: &oxfeed_common::webhook::Entity,
    ) -> oxfeed_common::Result<oxfeed_common::webhook::Entity> {
        Self::fetch(Method::POST, "/webhooks", webhook).await
    }

    pub async fn webhooks_update(
        id: &uuid::Uuid,
        webhook: &oxfeed_common::webhook::Entity,
    ) -> oxfeed_common::Result<oxfeed_common::webhook::Entity> {
        Self::fetch(Method::PUT, &format!("/webhooks/{}", id), webhook).await
    }

    pub async fn webhooks_delete(
        id: &uuid::Uuid,
    ) -> oxfeed_common::Result<oxfeed_common::webhook::Entity> {
        Self::fetch(
            Method::DELETE,
            &format!("/webhooks/{}", id),
            yew::format::Nothing,
        )
        .await
    }

    async fn fetch<B, R>(method: Method, url: &str, body: B) -> oxfeed_common::Result<R>
    where
        B: Into<Result<String, anyhow::Error>>,
        R: serde::de::DeserializeOwned,
    {
        let result = Self::try_fetch(method, url, body).await;

        if let Err(ref err) = result {
            let mut event_bus = crate::event::Bus::dispatcher();

            event_bus.send(err.into());
        }

        result
    }

    async fn try_fetch<B, R>(method: Method, url: &str, body: B) -> oxfeed_common::Result<R>
    where
        B: Into<Result<String, anyhow::Error>>,
        R: serde::de::DeserializeOwned,
    {
        let body: String = body.into().unwrap_or_default();
        let client = reqwest::Client::new();
        let response = client
            .request(method, &format!("{}{}", env!("API_URL"), url))
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", Self::token()))
            .body(body)
            .send()
            .await?;

        if response.status().is_server_error() {
            let error = response.text().await?;

            return Err(oxfeed_common::Error::Api(error));
        }

        match response.status() {
            http::status::StatusCode::UNAUTHORIZED => {
                let mut event_bus = crate::event::Bus::dispatcher();

                event_bus.send(crate::Event::AuthRequire);
                Err(oxfeed_common::Error::Auth)
            }
            http::status::StatusCode::NO_CONTENT => {
                serde_json::from_str("null").map_err(Into::into)
            }
            _ => {
                let data = response.json().await?;

                Ok(data)
            }
        }
    }
}
