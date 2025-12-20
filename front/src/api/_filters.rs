impl Api {
    pub async fn filters_all() -> oxfeed::Result<Vec<oxfeed::filter::Entity>> {
        Self::fetch(Method::GET, "/filters", ()).await
    }

    pub async fn filters_create(
        filter: &oxfeed::filter::Entity,
    ) -> oxfeed::Result<oxfeed::filter::Entity> {
        Self::fetch(Method::POST, "/filters", filter).await
    }

    pub async fn filters_update(
        id: &uuid::Uuid,
        filter: &oxfeed::filter::Entity,
    ) -> oxfeed::Result<oxfeed::filter::Entity> {
        Self::fetch(Method::PUT, &format!("/filters/{id}"), filter).await
    }

    pub async fn filters_delete(id: &uuid::Uuid) -> oxfeed::Result<oxfeed::filter::Entity> {
        Self::fetch(Method::DELETE, &format!("/filters/{id}"), ()).await
    }
}
