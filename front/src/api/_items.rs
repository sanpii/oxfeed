impl Api {
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
}
