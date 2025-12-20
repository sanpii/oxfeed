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

use reqwest::Method;

pub(crate) use call;

pub(crate) struct Api;

include!("_account.rs");
include!("_auth.rs");
include!("_filters.rs");
include!("_items.rs");
include!("_opml.rs");
include!("_sources.rs");
include!("_tags.rs");
include!("_token.rs");
include!("_webhooks.rs");

impl Api {
    pub async fn counts() -> oxfeed::Result<oxfeed::Counts> {
        Self::fetch(Method::GET, "/counts", ()).await
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
body_impl!(oxfeed::filter::Entity);
body_impl!(oxfeed::source::Entity);
body_impl!(oxfeed::webhook::Entity);
