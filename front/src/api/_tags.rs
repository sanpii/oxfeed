impl Api {
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
}
