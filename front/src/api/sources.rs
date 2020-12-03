impl<C> super::Api<C>
where
    C: yew::Component,
    <C as yew::Component>::Message: From<crate::event::Api>,
{
    pub fn sources_all(&mut self, pagination: &oxfeed_common::Pagination) {
        let url = format!("/sources?{}", pagination.to_query());

        self.fetch(
            super::Kind::Sources,
            http::Method::GET,
            &url,
            yew::format::Nothing,
        )
    }

    pub fn sources_create(&mut self, source: &oxfeed_common::source::Entity) {
        self.fetch(
            super::Kind::SourceCreate,
            http::Method::POST,
            "/sources",
            source,
        )
    }

    pub fn sources_update(&mut self, id: &uuid::Uuid, source: &oxfeed_common::source::Entity) {
        self.fetch(
            super::Kind::SourceUpdate,
            http::Method::PUT,
            &format!("/sources/{}", id),
            source,
        )
    }

    pub fn sources_delete(&mut self, id: &uuid::Uuid) {
        self.fetch(
            super::Kind::SourceDelete,
            http::Method::DELETE,
            &format!("/sources/{}", id),
            yew::format::Nothing,
        )
    }
}
