use oxfeed_common::item::Entity as Item;
use oxfeed_common::item::Model as ItemModel;
use oxfeed_common::source::Entity as Source;
use oxfeed_common::source::Model as SourceModel;
use oxfeed_common::webhook::Entity as Webhook;
use oxfeed_common::webhook::Model as WebhookModel;
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
            log::error!("{}", error);
        }
    }
}

impl actix::Actor for Actor {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        use actix::AsyncContext;

        let minutes = crate::env("UPDATE_INTERVAL")
            .unwrap_or_else(|_| "20".to_string())
            .parse()
            .unwrap_or(20);
        let interval = std::time::Duration::from_secs(60 * minutes);

        ctx.run_interval(interval, |act, _| {
            act.run();
        });
    }
}

impl actix::Supervised for Actor {}

struct Task;

impl Task {
    fn run(elephantry: &elephantry::Connection) -> oxfeed_common::Result {
        let sources = elephantry
            .find_where::<SourceModel>("active = $*", &[&true], None)?
            .collect::<Vec<_>>();

        sources.par_iter().for_each(|source| {
            let last_error = match Self::fetch(elephantry, source) {
                Ok(_) => None,
                Err(err) => {
                    log::error!("{}", err);
                    Some(err.to_string())
                }
            };

            if let Err(err) = elephantry.update_by_pk::<SourceModel>(
                &elephantry::pk! { source_id => source.id },
                &elephantry::values!(last_error),
            ) {
                log::error!("{}", err);
            }
        });

        elephantry.execute("refresh materialized view concurrently fts.item")?;

        Ok(())
    }

    fn fetch(elephantry: &elephantry::Connection, source: &Source) -> oxfeed_common::Result {
        log::info!("Fetching {}", source.url);

        let webhooks = elephantry
            .find_where::<WebhookModel>("webhook_id = any($*)", &[&source.webhooks], None)?
            .into_vec();

        let contents = attohttpc::RequestBuilder::try_new(attohttpc::Method::GET, &source.url)?
            .send()?
            .text()?;
        let feed = feed_rs::parser::parse(contents.as_bytes())?;
        let feed_icon = feed.icon.map(|x| x.uri);

        for entry in feed.entries {
            let link = entry.links[0].href.clone();

            let exist = elephantry
                .exist_where::<ItemModel>("link = $* and source_id = $*", &[&link, &source.id])?;

            if !exist {
                let title = entry
                    .title
                    .map(|x| x.content)
                    .unwrap_or_else(|| "&lt;no title&gt;".to_string());

                log::info!("Adding '{}'", title);

                let content = match entry.content {
                    Some(content) => content.body,
                    None => entry.summary.map(|x| x.content),
                };

                let mut item = Item {
                    id: None,
                    feed_id: entry.id,
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
                elephantry.insert_one::<ItemModel>(&item)?;
            }
        }

        Ok(())
    }

    fn icon(link: &str) -> Option<String> {
        let selector = scraper::Selector::parse("link[rel=\"icon\"]").unwrap();

        let request = match attohttpc::RequestBuilder::try_new(attohttpc::Method::GET, &link) {
            Ok(request) => request,
            Err(_) => return None,
        };

        let contents = match request.send() {
            Ok(contents) => contents.text().unwrap_or_default(),
            Err(_) => return None,
        };

        let html = scraper::Html::parse_document(&contents);
        let icon = match html.select(&selector).next() {
            Some(icon) => icon,
            None => return None,
        };
        let href = match icon.value().attr("href") {
            Some(href) => href.to_string(),
            None => return None,
        };

        if href.starts_with("http") {
            Some(href)
        } else {
            let mut url = match url::Url::parse(link) {
                Ok(url) => url,
                Err(_) => return None,
            };
            url.set_path("");

            Some(format!("{}{}", url, href.trim_start_matches('/')))
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

    fn call_webhook(webhook: &Webhook, item: &Item) -> oxfeed_common::Result {
        log::info!("call webhook '{}'", webhook.name);

        let response = attohttpc::RequestBuilder::try_new(attohttpc::Method::POST, &webhook.url)?
            .json(item)?
            .send()?;

        if response.is_success() {
            Ok(())
        } else {
            let error = format!(
                "{} · {}",
                response.status().to_string(),
                response.text().unwrap_or_default(),
            );
            Err(oxfeed_common::Error::Webhook(error))
        }
    }
}
