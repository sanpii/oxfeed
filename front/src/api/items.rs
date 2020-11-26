impl<C> super::Api<C> where C: yew::Component, <C as yew::Component>::Message: From<crate::event::Api> {
    pub fn items_all(
        &mut self,
        kind: &str,
        pagination: &crate::Pagination,
    ) {
        let kind = if kind == "all" {
            String::new()
        } else {
            kind.to_string()
        };

        let url = format!("/items/{}?page={}&limit={}", kind, pagination.page, pagination.limit);

        self.fetch(super::Kind::Items, http::Method::GET, &url, yew::format::Nothing)
    }

    pub fn items_content(
        &mut self,
        id: &str,
    ) {
        let url = format!("/items/{}/content", id);

        self.fetch(super::Kind::ItemContent, http::Method::GET, &url, yew::format::Nothing)
    }

    pub fn items_read(&mut self)
    {
        self.fetch(super::Kind::ItemsRead, http::Method::POST, "/items/read", yew::format::Nothing)
    }

    pub fn items_tag(
        &mut self,
        id: &str,
        key: &str,
        value: bool,
    ) {
        let url = format!("/items/{}", id);

        let json = serde_json::json!({
            key: value,
        });

        self.fetch(super::Kind::ItemPatch, http::Method::PATCH, &url, yew::format::Json(&json))
    }
}
