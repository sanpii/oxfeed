impl Api {
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
}
