impl<C> super::Api<C>
where
    C: yew::Component,
    <C as yew::Component>::Message: From<crate::event::Api>,
{
    pub fn tags_all(&mut self, pagination: &oxfeed_common::Pagination) {
        let url = format!("/tags?{}", pagination.to_query());

        self.fetch(
            super::Kind::Tags,
            http::Method::GET,
            &url,
            yew::format::Nothing,
        )
    }
}
