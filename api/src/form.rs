#[derive(serde::Deserialize)]
pub(crate) struct Source {
    source_id: Option<uuid::Uuid>,
    #[serde(default)]
    pub user_id: Option<uuid::Uuid>,
    url: String,
    title: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
}

impl std::convert::TryInto<oxfeed_common::source::Entity> for Source {
    type Error = crate::Error;

    fn try_into(self) -> crate::Result<oxfeed_common::source::Entity> {
        let user_id = match self.user_id {
            Some(user_id) => user_id,
            None => return Err(crate::Error::Auth),
        };

        let title = match self.title {
            Some(title) => title,
            None => self.title().unwrap_or_else(|| "<no title>".to_string()),
        };

        let entity = oxfeed_common::source::Entity {
            last_error: None,
            source_id: self.source_id,
            tags: self.tags,
            title,
            url: self.url.clone(),
            user_id,
        };

        Ok(entity)
    }
}

impl Source {
    fn title(&self) -> Option<String> {
        let contents = match Self::fetch(&self.url) {
            Ok(contents) => contents,
            Err(_) => return None,
        };

        let feed = match feed_rs::parser::parse(contents.as_bytes()) {
            Ok(feed) => feed,
            Err(_) => return None,
        };

        let mut title = feed.title.map(|x| x.content);

        if title.is_none() {
            for link in feed.links {
                let contents = match Self::fetch(&link.href) {
                    Ok(contents) => contents,
                    Err(_) => continue,
                };

                let html = scraper::Html::parse_document(&contents);

                let selector = scraper::Selector::parse("head title").unwrap();
                title = html.select(&selector).next().map(|x| x.inner_html());

                if title.is_some() {
                    break;
                }
            }
        }

        title
    }

    fn fetch(url: &str) -> Result<String, attohttpc::Error> {
        attohttpc::RequestBuilder::try_new(attohttpc::Method::GET, url)?
            .send()?
            .text()
    }
}
