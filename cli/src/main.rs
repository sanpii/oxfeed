mod errors;

use structopt::StructOpt;

pub use errors::Result;

use oxfeed_api::model::item::Model as ItemModel;
use oxfeed_api::model::source::Entity as Source;
use oxfeed_api::model::source::Model as SourceModel;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, default_value="/var/lock/oxfeed")]
    lock_file: String,
}

fn main() -> Result<()> {
    env_logger::init();

    let opt = Opt::from_args();

    let instance = single_instance::SingleInstance::new(&opt.lock_file).unwrap();
    if !instance.is_single() {
        log::warn!("Already running");
        return Ok(());
    }

    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable");
    let elephantry = elephantry::Pool::new(&database_url).expect("Unable to connect to postgresql");

    let sources = elephantry.find_all::<SourceModel>(None)?.collect::<Vec<_>>();

    sources.par_iter()
        .for_each(|source| {
            let last_error = match fetch(&elephantry, source) {
                Ok(_) => None,
                Err(err) => {
                    log::error!("{}", err);
                    Some(err.to_string())
                }
            };

            let mut data = std::collections::HashMap::new();
            data.insert("last_error".to_string(), &last_error as &dyn elephantry::ToSql);

            if let Err(err) = elephantry.update_by_pk::<SourceModel>(
                &elephantry::pk! { source_id => source.source_id },
                &data,
            ) {
                log::error!("{}", err);
            }
        });

    std::fs::remove_file(opt.lock_file)?;

    Ok(())
}

fn fetch(elephantry: &elephantry::Connection, source: &Source) -> Result<()> {
    log::info!("Fetching {}", source.url);

    let contents = attohttpc::RequestBuilder::try_new(attohttpc::Method::GET, &source.url)?
        .send()?
        .text()?;
    let feed = feed_rs::parser::parse(contents.as_bytes())?;

    for entry in feed.entries {
        let exist = elephantry.exist_where::<ItemModel>("id = $* and source_id = $*", &[&entry.id, &source.source_id])?;

        if !exist {
            let title = entry.title.map(|x| x.content).unwrap_or_else(|| "<no title>".to_string());

            log::info!("Adding '{}'", title);

            let item = oxfeed_api::model::item::Entity {
                item_id: None,
                id: entry.id,
                icon: icon(&entry.links),
                content: entry.summary.map(|x| x.content),
                title,
                published: entry.published,
                read: false,
                source_id: source.source_id.unwrap(),
                link: entry.links[0].href.clone(),
                favorite: false,
            };
            elephantry.insert_one::<ItemModel>(&item)?;
        }
    }

    Ok(())
}

fn icon(links: &[feed_rs::model::Link]) -> Option<String> {
    let selector = scraper::Selector::parse("link[rel=\"icon\"]").unwrap();

    for link in links {
        let request = match attohttpc::RequestBuilder::try_new(attohttpc::Method::GET, &link.href) {
            Ok(request) => request,
            Err(_) => continue,
        };

        let contents = match request.send() {
            Ok(contents) => contents.text().unwrap_or_default(),
            Err(_) => continue,
        };

        let html = scraper::Html::parse_document(&contents);
        match html.select(&selector).next() {
            Some(icon) => return icon.value().attr("href").map(|x| x.to_string()),
            None => continue,
        }
    }

    None
}
