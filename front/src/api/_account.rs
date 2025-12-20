impl Api {
    pub async fn account_create(user: &oxfeed::account::Entity) -> oxfeed::Result {
        Self::fetch(Method::POST, "/account", user).await
    }

    pub async fn account_delete() -> oxfeed::Result {
        Self::fetch(Method::DELETE, "/account", ()).await
    }

    pub async fn account_update(account: &oxfeed::account::Entity) -> oxfeed::Result {
        Self::fetch(Method::PUT, "/account", account).await
    }
}
