impl<C> super::Api<C>
where
    C: yew::Component,
    <C as yew::Component>::Message: From<crate::event::Api>,
{
    pub fn opml_import(&mut self, opml: Result<std::string::String, anyhow::Error>)
    where
        C: yew::Component,
        <C as yew::Component>::Message: std::convert::TryFrom<crate::event::Api>,
    {
        self.fetch(super::Kind::OpmlImport, http::Method::POST, "/opml", opml)
    }
}
