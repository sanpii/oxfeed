#[derive(serde::Deserialize)]
pub(crate) struct Source {
    id: Option<uuid::Uuid>,
    #[serde(default)]
    pub user_id: Option<uuid::Uuid>,
    url: String,
    title: Option<String>,
    active: bool,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    webhooks: Vec<uuid::Uuid>,
}

impl std::convert::TryInto<oxfeed_common::source::Entity> for Source {
    type Error = oxfeed_common::Error;

    fn try_into(self) -> oxfeed_common::Result<oxfeed_common::source::Entity> {
        let user_id = self.user_id.ok_or(oxfeed_common::Error::Auth)?;

        let title = self
            .title
            .clone()
            .or_else(|| self.title())
            .unwrap_or_else(|| "<no title>".to_string());

        let entity = oxfeed_common::source::Entity {
            last_error: None,
            id: self.id,
            tags: self.tags,
            title,
            url: self.url.clone(),
            user_id,
            active: self.active,
            webhooks: self.webhooks,
        };

        Ok(entity)
    }
}

impl Source {
    fn title(&self) -> Option<String> {
        let Ok(contents) = Self::fetch(&self.url) else {
            return None;
        };

        let Ok(feed) = feed_rs::parser::parse(contents.as_bytes()) else {
            return None;
        };

        let mut title = feed.title.map(|x| x.content);

        if title.is_none() {
            for link in feed.links {
                let Ok(contents) = Self::fetch(&link.href) else {
                    continue;
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

#[derive(serde::Deserialize)]
pub(crate) struct Webhook {
    #[serde(default)]
    id: Option<uuid::Uuid>,
    #[serde(default)]
    pub user_id: Option<uuid::Uuid>,
    url: String,
    name: String,
    mark_read: bool,
}

impl std::convert::TryInto<oxfeed_common::webhook::Entity> for Webhook {
    type Error = oxfeed_common::Error;

    fn try_into(self) -> oxfeed_common::Result<oxfeed_common::webhook::Entity> {
        let user_id = match self.user_id {
            Some(user_id) => Some(user_id),
            None => return Err(oxfeed_common::Error::Auth),
        };

        let entity = oxfeed_common::webhook::Entity {
            id: self.id,
            name: self.name.clone(),
            url: self.url.clone(),
            user_id,
            last_error: None,
            mark_read: self.mark_read,
        };

        Ok(entity)
    }
}
