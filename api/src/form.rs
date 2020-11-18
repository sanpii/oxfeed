#[derive(serde::Deserialize)]
pub(crate) struct Source {
    source_id: Option<uuid::Uuid>,
    url: String,
    #[serde(default)]
    tags: Vec<String>,
}

impl std::convert::TryInto<crate::model::source::Entity> for Source {
    type Error = crate::Error;

    fn try_into(self) -> crate::Result<crate::model::source::Entity> {
        let contents = attohttpc::get(&self.url).send()?.text()?;
        let feed = feed_rs::parser::parse(contents.as_bytes())?;

        let mut title = feed.title.map(|x| x.content);

        if title.is_none() {
            for link in feed.links {
                let contents = attohttpc::get(&link.href).send()?.text()?;
                let html = scraper::Html::parse_document(&contents);

                let selector = scraper::Selector::parse("head title").unwrap();
                title = html.select(&selector).next().map(|x| x.inner_html());

                if title.is_some() {
                    break;
                }
            }
        }

        let entity = crate::model::source::Entity {
            url: self.url.clone(),
            source_id: self.source_id,
            title: title.unwrap_or_else(|| "<no title>".to_string()),
            tags: self.tags,
        };

        Ok(entity)
    }
}
