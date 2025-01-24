use reqwest::Method;

macro_rules! call {
    ($context:expr, $fn:ident, $($args:expr),*) => {{
        $context.dispatch($crate::Action::Fetch);
        let result = $crate::Api::$fn( $( $args, )* ).await;
        $context.dispatch($crate::Action::Fetched);

        match result {
            Ok(result) => result,
            Err(err) if matches!(err, oxfeed::Error::Auth) => {
                $context.dispatch(crate::Action::AuthRequire);
                return;
            }
            Err(err) => {
                $context.dispatch(err.into());
                return;
            }
        }
    }};
    ($context:expr, $fn:ident) => {
        $crate::api::call!($context, $fn, )
    };
}

pub(crate) use call;

pub(crate) struct Api;

impl Api {
    #[must_use]
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

    pub async fn counts() -> oxfeed::Result<oxfeed::Counts> {
        Self::fetch(Method::GET, "/counts", ()).await
    }

    pub async fn auth() -> oxfeed::Result<oxfeed::user::Entity> {
        Self::fetch(Method::GET, "/auth", ()).await
    }

    pub async fn auth_login(email: &str, password: &str, remember_me: &bool) -> oxfeed::Result {
        use hmac::Mac;
        use jwt::SignWithKey;

        let key: hmac::Hmac<sha2::Sha256> =
            hmac::Hmac::new_from_slice(env!("SECRET").as_bytes()).unwrap();
        let mut claims = std::collections::BTreeMap::new();
        claims.insert("email", email);
        claims.insert("password", password);

        let token = claims.sign_with_key(&key).unwrap();

        let data: String = Self::fetch(Method::POST, "/auth/login", token).await?;

        Self::set_token(&data, *remember_me);

        Ok(())
    }

    #[allow(dependency_on_unit_never_type_fallback)]
    pub async fn auth_logout() -> oxfeed::Result {
        Self::fetch(Method::POST, "/auth/logout", ()).await?;

        Self::clear_token();

        Ok(())
    }

    pub async fn items_all(
        kind: &str,
        pagination: &elephantry_extras::Pagination,
    ) -> oxfeed::Result<crate::Pager<oxfeed::item::Item>> {
        let kind = if kind == "all" {
            String::new()
        } else {
            kind.to_string()
        };

        let url = format!("/items/{kind}?{}", pagination.to_query());

        Self::fetch(Method::GET, &url, ()).await
    }

    pub async fn items_content(id: &uuid::Uuid) -> oxfeed::Result<String> {
        let url = format!("/items/{id}/content");

        Self::fetch(Method::GET, &url, ()).await
    }

    pub async fn items_read() -> oxfeed::Result {
        Self::fetch(Method::POST, "/items/read", ()).await
    }

    pub async fn items_tag(id: &uuid::Uuid, key: &str, value: bool) -> oxfeed::Result {
        let url = format!("/items/{id}");

        let json = serde_json::json!({
            key: value,
        });

        Self::fetch(Method::PATCH, &url, Body::Json(json)).await
    }

    pub async fn items_search(
        what: &str,
        filter: &crate::Filter,
        pagination: &elephantry_extras::Pagination,
    ) -> oxfeed::Result<crate::Pager<oxfeed::item::Item>> {
        let url = format!(
            "/search/{what}?{}&{}",
            filter.to_url_param(),
            pagination.to_query()
        );

        Self::fetch(Method::GET, &url, ()).await
    }

    pub async fn opml_import(opml: String) -> oxfeed::Result {
        Self::fetch(Method::POST, "/opml", opml).await
    }

    pub async fn sources_all(
        pagination: &elephantry_extras::Pagination,
    ) -> oxfeed::Result<crate::Pager<oxfeed::source::Entity>> {
        let url = format!("/sources?{}", pagination.to_query());

        Self::fetch(Method::GET, &url, ()).await
    }

    pub async fn sources_create(
        source: &oxfeed::source::Entity,
    ) -> oxfeed::Result<oxfeed::source::Entity> {
        Self::fetch(Method::POST, "/sources", source).await
    }

    pub async fn sources_update(
        id: &uuid::Uuid,
        source: &oxfeed::source::Entity,
    ) -> oxfeed::Result<oxfeed::source::Entity> {
        Self::fetch(Method::PUT, &format!("/sources/{id}"), source).await
    }

    pub async fn sources_delete(id: &uuid::Uuid) -> oxfeed::Result<oxfeed::source::Entity> {
        Self::fetch(Method::DELETE, &format!("/sources/{id}"), ()).await
    }

    pub async fn sources_search(
        filter: &crate::Filter,
        pagination: &elephantry_extras::Pagination,
    ) -> oxfeed::Result<crate::Pager<oxfeed::source::Entity>> {
        let url = format!(
            "/search/sources?{}&{}",
            filter.to_url_param(),
            pagination.to_query()
        );

        Self::fetch(Method::GET, &url, ()).await
    }

    pub async fn tags_all(
        pagination: &elephantry_extras::Pagination,
    ) -> oxfeed::Result<Vec<oxfeed::Tag>> {
        let url = format!("/tags?{}", pagination.to_query());

        Self::fetch(Method::GET, &url, ()).await
    }

    pub async fn tags_search(
        filter: &crate::Filter,
        pagination: &elephantry_extras::Pagination,
    ) -> oxfeed::Result<crate::Pager<String>> {
        let url = format!(
            "/search/tags?{}&{}",
            filter.to_url_param(),
            pagination.to_query()
        );

        Self::fetch(Method::GET, &url, ()).await
    }

    pub async fn tags_rename(tag: &str, name: &str) -> oxfeed::Result {
        let url = format!("/tags/{tag}");

        Self::fetch(Method::POST, &url, name).await
    }

    pub async fn account_create(user: &oxfeed::account::Entity) -> oxfeed::Result {
        Self::fetch(Method::POST, "/account", user).await
    }

    pub async fn account_delete() -> oxfeed::Result {
        Self::fetch(Method::DELETE, "/account", ()).await
    }

    pub async fn account_update(account: &oxfeed::account::Entity) -> oxfeed::Result {
        Self::fetch(Method::PUT, "/account", account).await
    }

    pub async fn webhooks_all() -> oxfeed::Result<Vec<oxfeed::webhook::Entity>> {
        Self::fetch(Method::GET, "/webhooks", ()).await
    }

    pub async fn webhooks_create(
        webhook: &oxfeed::webhook::Entity,
    ) -> oxfeed::Result<oxfeed::webhook::Entity> {
        Self::fetch(Method::POST, "/webhooks", webhook).await
    }

    pub async fn webhooks_update(
        id: &uuid::Uuid,
        webhook: &oxfeed::webhook::Entity,
    ) -> oxfeed::Result<oxfeed::webhook::Entity> {
        Self::fetch(Method::PUT, &format!("/webhooks/{id}"), webhook).await
    }

    pub async fn webhooks_delete(id: &uuid::Uuid) -> oxfeed::Result<oxfeed::webhook::Entity> {
        Self::fetch(Method::DELETE, &format!("/webhooks/{id}"), ()).await
    }

    async fn fetch<B, R>(method: Method, url: &str, body: B) -> oxfeed::Result<R>
    where
        B: Into<Body>,
        R: serde::de::DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let response = client
            .request(method, format!("{}{url}", env!("API_URL")))
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", Self::token()))
            .body(body.into().to_string())
            .send()
            .await?;

        let status = response.status();

        if status.is_server_error() {
            let error = response.text().await?;

            return Err(oxfeed::Error::Api(error));
        }

        match status {
            reqwest::StatusCode::UNAUTHORIZED => Err(oxfeed::Error::Auth),
            reqwest::StatusCode::NO_CONTENT => serde_json::from_str("null").map_err(Into::into),
            reqwest::StatusCode::FORBIDDEN => Err(oxfeed::Error::InvalidLogin),
            _ => {
                let data = response.json().await?;

                Ok(data)
            }
        }
    }
}

pub(crate) enum Body {
    Empty,
    Json(serde_json::Value),
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Empty => String::new(),
            Self::Json(json) => json.to_string(),
        };

        f.write_str(&s)
    }
}

impl From<String> for Body {
    fn from(value: String) -> Self {
        Self::Json(serde_json::Value::String(value))
    }
}

impl From<&str> for Body {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<()> for Body {
    fn from(_: ()) -> Self {
        Self::Empty
    }
}

macro_rules! body_impl {
    ($ty:ty) => {
        impl From<&$ty> for Body {
            fn from(entity: &$ty) -> Self {
                Body::Json(serde_json::to_value(entity).unwrap())
            }
        }
    };
}

body_impl!(oxfeed::account::Entity);
body_impl!(oxfeed::source::Entity);
body_impl!(oxfeed::webhook::Entity);
