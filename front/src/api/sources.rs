impl<C> super::Api<C> where C: yew::Component, <C as yew::Component>::Message: From<crate::event::Api> {
    pub fn sources_all(
        &mut self,
        pagination: &crate::Pagination,
    ) {
        let url = format!("/sources?page={}&limit={}", pagination.page, pagination.limit);

        self.fetch(super::Kind::Sources, http::Method::GET, &url, yew::format::Nothing)
    }

    pub fn sources_create(
        &mut self,
        source: &crate::Source,
    ) {
        self.fetch(super::Kind::SourceCreate, http::Method::POST, "/sources", source)
    }

    pub fn sources_update(
        &mut self,
        id: &str,
        source: &crate::Source,
    ) {
        self.fetch(super::Kind::SourceUpdate, http::Method::PUT, &format!("/sources/{}", id), source)
    }

    pub fn sources_delete(
        &mut self,
        id: &str,
    ) {
        self.fetch(super::Kind::SourceDelete, http::Method::DELETE, &format!("/sources/{}", id), yew::format::Nothing)
    }
}
