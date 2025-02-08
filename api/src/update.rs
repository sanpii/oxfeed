use oxfeed::item::Entity as Item;
use oxfeed::item::Model as ItemModel;
use oxfeed::media::Entity as Media;
use oxfeed::media::Model as MediaModel;
use oxfeed::source::Entity as Source;
use oxfeed::source::Model as SourceModel;
use oxfeed::webhook::Entity as Webhook;
use oxfeed::webhook::Model as WebhookModel;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub(crate) struct Actor {
    elephantry: elephantry::Pool,
}

impl Actor {
    pub fn new(elephantry: &elephantry::Pool) -> Self {
        Self {
            elephantry: elephantry.clone(),
        }
    }

    pub fn start(self) -> actix::Addr<Self> {
        actix::Supervisor::start(|_| self)
    }
}

impl actix::Actor for Actor {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        use actix::AsyncContext as _;

        let minutes = envir::parse("UPDATE_INTERVAL").unwrap_or(20);
        let interval = std::time::Duration::from_secs(60 * minutes);

        ctx.run_interval(interval, |_, ctx| {
            ctx.notify(Signal);
        });
    }
}

impl actix::Supervised for Actor {}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Signal;

impl actix::Handler<Signal> for Actor {
    type Result = actix::ResponseActFuture<Self, ()>;

    fn handle(&mut self, _: Signal, _: &mut actix::Context<Self>) -> Self::Result {
        use actix::ActorFutureExt as _;
        use actix::WrapFuture as _;

        log::warn!("Start update");

        Box::pin(
            Task::run(self.elephantry.get_default().unwrap().clone())
                .into_actor(self)
                .map(|res, _, _| {
                    if let Err(error) = res {
                        log::error!("{error}");
                    }
                    log::warn!("Update finished");
                }),
        )
    }
}

struct Task;

impl Task {
    async fn run(elephantry: elephantry::Connection) -> oxfeed::Result {
        let sources = elephantry
            .find_where::<SourceModel>("active", &[], None)?
            .collect::<Vec<_>>();

        let handles = sources
            .par_iter()
            .map(|source| async {
                let source = source.clone();
                let elephantry = elephantry.clone();

                let last_error = match Self::fetch(&elephantry, &source).await {
                    Ok(_) => None,
                    Err(err) => {
                        log::error!("{err}");
                        Some(err.to_string())
                    }
                };

                if let Err(err) = elephantry.update_by_pk::<SourceModel>(
                    &elephantry::pk! { source_id => source.id },
                    &elephantry::values!(last_error),
                ) {
                    log::error!("{err}");
                }
            })
            .collect::<Vec<_>>();

        futures_util::future::join_all(handles).await;

        elephantry.execute("refresh materialized view concurrently fts.item")?;

        Ok(())
    }

    async fn fetch(elephantry: &elephantry::Connection, source: &Source) -> oxfeed::Result {
        log::info!("Fetching {}", source.url);

        let response = reqwest::Client::new()
            .get(&source.url)
            .timeout(std::time::Duration::from_secs(5 * 60))
            .send()
            .await?;

        if !Self::is_modified(response.headers(), elephantry, source).unwrap_or(true) {
            return Ok(());
        }

        let contents = response.text().await?;

        let webhooks = elephantry
            .find_where::<WebhookModel>("webhook_id = any($*)", &[&source.webhooks], None)?
            .into_vec();

        let feed = feed_rs::parser::parse(contents.as_bytes())?;
        Self::update_source(&elephantry, &source, &feed).await?;

        for entry in feed.entries {
            let link = match entry.links.first() {
                Some(link) => link.href.clone(),
                None => continue,
            };

            let exist = elephantry
                .exist_where::<ItemModel>("link = $* and source_id = $*", &[&link, &source.id])?;

            if !exist {
                let title = entry
                    .title
                    .map_or_else(|| "&lt;no title&gt;".to_string(), |x| x.content);

                log::info!("Adding '{title}'");

                let content = match entry.content {
                    Some(content) => content.body,
                    None => entry.summary.map(|x| x.content),
                };

                let mut item = Item {
                    id: None,
                    content,
                    title,
                    published: entry.published,
                    read: false,
                    source_id: source.id.unwrap(),
                    link,
                    favorite: false,
                };

                item.read = Self::call_webhooks(elephantry, &webhooks, &item);
                item = elephantry.insert_one::<ItemModel>(&item)?;

                for media in entry.media {
                    Self::create_media(elephantry, &item, &media.content)?;
                }
            }
        }

        Ok(())
    }

    fn is_modified(
        headers: &reqwest::header::HeaderMap,
        elephantry: &elephantry::Connection,
        source: &Source,
    ) -> std::result::Result<bool, Box<dyn std::error::Error>> {
        let last_modified = match headers.get(reqwest::header::LAST_MODIFIED) {
            Some(last_modified) => last_modified.to_str()?,
            None => return Ok(true),
        };

        let last_modified = chrono::DateTime::parse_from_rfc2822(last_modified)?;

        let query = "select published from item join source using(source_id) where source_id = $* order by 1 desc limit 1;";
        let last_item =
            elephantry.query_one::<chrono::DateTime<chrono::FixedOffset>>(query, &[&source.id])?;

        Ok(last_item > last_modified)
    }

    async fn update_source(
        elephantry: &elephantry::Connection,
        source: &Source,
        feed: &feed_rs::model::Feed,
    ) -> oxfeed::Result {
        let icon = Self::icon(feed).await;
        let title = feed
            .title
            .as_ref()
            .map(|x| x.content.clone())
            .unwrap_or_default();

        let mut values = elephantry::values!(icon);

        if source.title.is_empty() {
            values.insert("title".to_string(), &title);
        }

        if let Err(err) = elephantry
            .update_by_pk::<SourceModel>(&elephantry::pk! { source_id => source.id }, &values)
        {
            log::error!("{err}");
        }

        Ok(())
    }

    fn create_media(
        elephantry: &elephantry::Connection,
        item: &Item,
        medias: &[feed_rs::model::MediaContent],
    ) -> oxfeed::Result {
        for media in medias {
            let Some(content_type) = media.content_type.as_ref().map(ToString::to_string) else {
                continue;
            };

            if content_type.starts_with("audio/") || content_type.starts_with("video/") {
                let media = Media {
                    id: None,
                    item_id: item.id.unwrap(),
                    url: media.url.as_ref().unwrap().to_string(),
                    content_type: Some(content_type),
                };

                elephantry.insert_one::<MediaModel>(&media)?;
            }
        }

        Ok(())
    }

    async fn icon(feed: &feed_rs::model::Feed) -> Option<String> {
        if let Some(icon) = feed.icon.as_ref().map(|x| x.uri.clone()) {
            return Some(icon);
        }

        if let Some(link) = feed.links.get(0) {
            if let Some(icon) = Self::favicon(&link.href).await {
                return Some(icon);
            }
        }

        if let Some(link) = feed.entries.get(0).and_then(|x| x.links.get(0)) {
            Self::favicon(&link.href).await
        } else {
            None
        }
    }

    async fn favicon(link: &str) -> Option<String> {
        let selector = scraper::Selector::parse("link[rel=\"icon\"]").unwrap();

        let Ok(response) = reqwest::get(link).await else {
            return None;
        };

        let Ok(contents) = response.text().await else {
            return None;
        };

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

    fn call_webhooks(
        elephantry: &elephantry::Connection,
        webhooks: &[Webhook],
        item: &Item,
    ) -> bool {
        let mut read = false;

        for webhook in webhooks {
            match Self::call_webhook(webhook, item) {
                Ok(_) => read |= webhook.mark_read,
                Err(err) => {
                    let last_error = err.to_string();
                    elephantry
                        .update_by_pk::<WebhookModel>(
                            &elephantry::pk! { webhook_id => webhook.id },
                            &elephantry::values!(last_error),
                        )
                        .ok();
                }
            }
        }

        read
    }

    fn call_webhook(webhook: &Webhook, item: &Item) -> oxfeed::Result {
        log::info!("call webhook '{}'", webhook.name);

        let response = reqwest::blocking::Client::new()
            .post(&webhook.url)
            .timeout(std::time::Duration::from_secs(5 * 60))
            .json(item)
            .send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error = format!(
                "{} · {}",
                response.status(),
                response.text().unwrap_or_default(),
            );
            Err(oxfeed::Error::Webhook(error))
        }
    }
}
