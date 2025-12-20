#[derive(serde::Deserialize)]
pub(crate) struct Filter {
    #[serde(default)]
    id: Option<uuid::Uuid>,
    #[serde(default)]
    pub user_id: Option<uuid::Uuid>,
    name: String,
    regex: String,
}

impl std::convert::TryInto<oxfeed::filter::Entity> for Filter {
    type Error = oxfeed::Error;

    fn try_into(self) -> oxfeed::Result<oxfeed::filter::Entity> {
        let user_id = match self.user_id {
            Some(user_id) => Some(user_id),
            None => return Err(oxfeed::Error::Auth),
        };

        let entity = oxfeed::filter::Entity {
            id: self.id,
            name: self.name.clone(),
            regex: self.regex.clone(),
            user_id,
        };

        Ok(entity)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct Source {
    id: Option<uuid::Uuid>,
    #[serde(default)]
    pub user_id: Option<uuid::Uuid>,
    url: String,
    title: Option<String>,
    active: bool,
    #[serde(default)]
    filters: Vec<uuid::Uuid>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    webhooks: Vec<uuid::Uuid>,
}

impl std::convert::TryInto<oxfeed::source::Entity> for Source {
    type Error = oxfeed::Error;

    fn try_into(self) -> oxfeed::Result<oxfeed::source::Entity> {
        let user_id = self.user_id.ok_or(oxfeed::Error::Auth)?;

        let title = self
            .title
            .clone()
            .or_else(|| self.title())
            .unwrap_or_else(|| "<no title>".to_string());

        let entity = oxfeed::source::Entity {
            last_error: None,
            id: self.id,
            tags: self.tags,
            title,
            icon: None,
            url: self.url.clone(),
            user_id,
            active: self.active,
            filters: self.filters,
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

    fn fetch(url: &str) -> Result<String, reqwest::Error> {
        reqwest::blocking::get(url)?.text()
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

impl std::convert::TryInto<oxfeed::webhook::Entity> for Webhook {
    type Error = oxfeed::Error;

    fn try_into(self) -> oxfeed::Result<oxfeed::webhook::Entity> {
        let user_id = match self.user_id {
            Some(user_id) => Some(user_id),
            None => return Err(oxfeed::Error::Auth),
        };

        let entity = oxfeed::webhook::Entity {
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
