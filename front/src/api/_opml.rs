impl Api {
    pub async fn opml_import(opml: String) -> oxfeed::Result {
        Self::fetch(Method::POST, "/opml", opml).await
    }
}
