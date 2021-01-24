use structopt::StructOpt;

use oxfeed_common::item::Entity as Item;
use oxfeed_common::item::Model as ItemModel;
use oxfeed_common::source::Entity as Source;
use oxfeed_common::source::Model as SourceModel;
use oxfeed_common::webhook::Entity as Webhook;
use oxfeed_common::webhook::Model as WebhookModel;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, default_value = "/var/lock/oxfeed")]
    lock_file: String,
    source_id: Vec<uuid::Uuid>,
}

fn main() -> oxfeed_common::Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    env_logger::init();

    let opt = Opt::from_args();

    let instance = single_instance::SingleInstance::new(&opt.lock_file).unwrap();
    if !instance.is_single() {
        log::warn!("Already running");
        return Ok(());
    }

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable");
    let elephantry = elephantry::Pool::new(&database_url).expect("Unable to connect to postgresql");

    let sources = if opt.source_id.is_empty() {
        elephantry
            .find_where::<SourceModel>("active = $*", &[&true], None)?
            .collect::<Vec<_>>()
    } else {
        elephantry
            .find_where::<SourceModel>("source_id = any($*) and active = $*", &[&opt.source_id, &true], None)?
            .collect::<Vec<_>>()
    };

    sources.par_iter().for_each(|source| {
        let last_error = match fetch(&elephantry, source) {
            Ok(_) => None,
            Err(err) => {
                log::error!("{}", err);
                Some(err.to_string())
            }
        };

        let mut data = std::collections::HashMap::new();
        data.insert(
            "last_error".to_string(),
            &last_error as &dyn elephantry::ToSql,
        );

        if let Err(err) = elephantry
            .update_by_pk::<SourceModel>(&elephantry::pk! { source_id => source.source_id }, &data)
        {
            log::error!("{}", err);
        }
    });

    std::fs::remove_file(opt.lock_file)?;

    Ok(())
}

fn fetch(elephantry: &elephantry::Connection, source: &Source) -> oxfeed_common::Result<()> {
    log::info!("Fetching {}", source.url);

    let webhooks = elephantry.find_where::<WebhookModel>("webhook_id = any($*)", &[&source.webhooks], None)?
        .into_vec();

    let contents = attohttpc::RequestBuilder::try_new(attohttpc::Method::GET, &source.url)?
        .send()?
        .text()?;
    let feed = feed_rs::parser::parse(contents.as_bytes())?;
    let feed_icon = feed.icon.map(|x| x.uri);

    for entry in feed.entries {
        let exist = elephantry.exist_where::<ItemModel>(
            "id = $* and source_id = $*",
            &[&entry.id, &source.source_id],
        )?;

        if !exist {
            let title = entry
                .title
                .map(|x| x.content)
                .unwrap_or_else(|| "<no title>".to_string());

            log::info!("Adding '{}'", title);
            let link = entry.links[0].href.clone();

            let content = match entry.content {
                Some(content) => content.body,
                None => entry.summary.map(|x| x.content),
            };

            let mut item = Item {
                item_id: None,
                id: entry.id,
                icon: feed_icon.clone().or_else(|| icon(&link)),
                content,
                title,
                published: entry.published,
                read: false,
                source_id: source.source_id.unwrap(),
                link,
                favorite: false,
            };

            if elephantry.upsert_one::<ItemModel>(&item, "(link)", "nothing")?.is_some() {
                call_webhooks(&elephantry, &webhooks, &mut item);
            }
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
        let mut url = match url::Url::parse(&link) {
            Ok(url) => url,
            Err(_) => return None,
        };
        url.set_path("");

        Some(format!("{}{}", url, href.trim_start_matches('/')))
    }
}

fn call_webhooks(elephantry: &elephantry::Connection, webhooks: &[Webhook], item: &mut Item) {
    let mut read = false;

    for webhook in webhooks {
        match call_webhook(&webhook, &item) {
            Ok(_) => read |= webhook.mark_read,
            Err(err) => {
                let last_error = err.to_string();
                elephantry.update_by_pk::<WebhookModel>(
                    &elephantry::pk! { webhook_id => webhook.webhook_id },
                    &elephantry::values!(last_error),
                ).ok();
            }
        }
    }

    item.read = read;
}

fn call_webhook(webhook: &Webhook, item: &Item) -> oxfeed_common::Result<()> {
    log::info!("call webhook '{}'", webhook.name);

    let response = attohttpc::RequestBuilder::try_new(attohttpc::Method::POST, &webhook.url)?
        .json(item)?
        .send()?;

    if response.is_success() {
        Ok(())
    } else {
        Err(oxfeed_common::Error::Webhook(response.status().to_string()))
    }
}
