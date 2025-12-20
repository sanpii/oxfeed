impl Api {
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
}
