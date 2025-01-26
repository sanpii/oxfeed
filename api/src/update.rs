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

    fn run(&self) {
        if let Err(error) = Task::run(&self.elephantry) {
            log::error!("{error}");
        }
    }
}

impl actix::Actor for Actor {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        use actix::AsyncContext;

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
    type Result = ();

    fn handle(&mut self, _: Signal, ctx: &mut actix::Context<Self>) {
        use actix::AsyncContext;

        ctx.run_later(std::time::Duration::from_secs(0), |act, _| {
            log::warn!("Start update");
            act.run();
            log::warn!("Update finished");
        });
    }
}

struct Task;

impl Task {
    fn run(elephantry: &elephantry::Connection) -> oxfeed::Result {
        let sources = elephantry
            .find_where::<SourceModel>("active", &[], None)?
            .collect::<Vec<_>>();

        sources.par_iter().for_each(|source| {
            let last_error = match Self::fetch(elephantry, source) {
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
        });

        elephantry.execute("refresh materialized view concurrently fts.item")?;

        Ok(())
    }

    fn fetch(elephantry: &elephantry::Connection, source: &Source) -> oxfeed::Result {
        log::info!("Fetching {}", source.url);

        let response = attohttpc::RequestBuilder::try_new(attohttpc::Method::GET, &source.url)?
            .timeout(std::time::Duration::from_secs(5 * 60))
            .send()?;

        if !Self::is_modified(response.headers(), elephantry, source).unwrap_or_default() {
            return Ok(());
        }

        let contents = response.text()?;

        let webhooks = elephantry
            .find_where::<WebhookModel>("webhook_id = any($*)", &[&source.webhooks], None)?
            .into_vec();

        let feed = feed_rs::parser::parse(contents.as_bytes())?;
        let feed_icon = feed.icon.map(|x| x.uri);

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
                    icon: feed_icon.clone().or_else(|| Self::icon(&link)),
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
        headers: &attohttpc::header::HeaderMap,
        elephantry: &elephantry::Connection,
        source: &Source,
    ) -> std::result::Result<bool, Box<dyn std::error::Error>> {
        let last_modified = match headers.get(attohttpc::header::LAST_MODIFIED) {
            Some(last_modified) => last_modified.to_str()?,
            None => return Ok(true),
        };

        let last_modified = chrono::DateTime::parse_from_rfc2822(last_modified)?;

        let query = "select published from item join source using(source_id) where source_id = $* order by 1 desc limit 1;";
        let last_item =
            elephantry.query_one::<chrono::DateTime<chrono::FixedOffset>>(query, &[&source.id])?;

        Ok(last_item > last_modified)
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

    fn icon(link: &str) -> Option<String> {
        let selector = scraper::Selector::parse("link[rel=\"icon\"]").unwrap();

        let Ok(request) = attohttpc::RequestBuilder::try_new(attohttpc::Method::GET, link) else {
            return None;
        };

        let contents = match request.send() {
            Ok(contents) => contents.text().unwrap_or_default(),
            Err(_) => return None,
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

        let response = attohttpc::RequestBuilder::try_new(attohttpc::Method::POST, &webhook.url)?
            .timeout(std::time::Duration::from_secs(5 * 60))
            .json(item)?
            .send()?;

        if response.is_success() {
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
