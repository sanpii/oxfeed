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
    language: Option<String>,
    active: bool,
    #[serde(default)]
    filters: Vec<uuid::Uuid>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    webhooks: Vec<uuid::Uuid>,
}

impl Source {
    pub async fn into_entity(self) -> oxfeed::Result<oxfeed::source::Entity> {
        let user_id = self.user_id.ok_or(oxfeed::Error::Auth)?;
        let feed = self.feed().await?;

        let title = match self.title {
            Some(ref title) if !title.is_empty() => title.clone(),
            _ => self
                .title(&feed)
                .await
                .unwrap_or_else(|| "<no title>".to_string()),
        };

        let icon = self.icon(&feed).await;

        let language = self.language.unwrap_or_else(|| "simple".to_string());

        let entity = oxfeed::source::Entity {
            last_error: None,
            id: self.id,
            tags: self.tags,
            title,
            language,
            icon,
            url: self.url.clone(),
            user_id,
            active: self.active,
            filters: self.filters,
            webhooks: self.webhooks,
        };

        Ok(entity)
    }

    async fn title(&self, feed: &feed_rs::model::Feed) -> Option<String> {
        let mut title = feed.title.as_ref().map(|x| x.content.clone());

        if title.is_none() {
            for link in &feed.links {
                let Ok(contents) = Self::fetch(&link.href).await else {
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

    async fn fetch(url: &str) -> Result<String, reqwest::Error> {
        let response = reqwest::Client::new()
            .get(url)
            .timeout(std::time::Duration::from_mins(5))
            .send()
            .await?;

        response.text().await
    }

    async fn feed(&self) -> oxfeed::Result<feed_rs::model::Feed> {
        let contents = Self::fetch(&self.url).await?;
        let feed = feed_rs::parser::parse(contents.as_bytes())?;

        Ok(feed)
    }

    async fn icon(&self, feed: &feed_rs::model::Feed) -> Option<String> {
        if let Some(icon) = feed.icon.as_ref().map(|x| x.uri.clone()) {
            return Some(icon);
        }

        if let Some(link) = feed.links.first()
            && let Some(icon) = Self::favicon(&link.href).await
        {
            return Some(icon);
        }

        if let Some(link) = feed.entries.first().and_then(|x| x.links.first())
            && let Some(icon) = Self::favicon(&link.href).await
        {
            return Some(icon);
        }

        self.default_favicon().await
    }

    async fn favicon(link: &str) -> Option<String> {
        let selector = scraper::Selector::parse("link[rel=\"icon\"]").unwrap();

        let response = reqwest::get(link).await.ok()?;
        let contents = response.text().await.ok()?;

        let html = scraper::Html::parse_document(&contents);
        let icon = html.select(&selector).next()?;
        let href = match icon.value().attr("href") {
            Some(href) => href.to_string(),
            None => return None,
        };

        if href.starts_with("http") {
            Some(href)
        } else {
            let Ok(mut url) = url::Url::parse(link) else {
                return None;
            };
            url.set_path("");

            Some(format!("{url}{}", href.trim_start_matches('/')))
        }
    }

    async fn default_favicon(&self) -> Option<String> {
        let url = url::Url::parse(&self.url).ok()?;
        let favicon = format!("{}/favicon.ico", url.origin().ascii_serialization());
        let request = reqwest::get(&favicon).await.ok()?;

        if request.status().is_success() {
            Some(favicon.clone())
        } else {
            None
        }
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
