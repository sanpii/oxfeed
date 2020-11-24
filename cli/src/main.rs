use oxfeed_api::model::item::Model as ItemModel;
use oxfeed_api::model::source::Model as SourceModel;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    env_logger::init();

    let instance = single_instance::SingleInstance::new("oxfeed").unwrap();
    if !instance.is_single() {
        log::warn!("Already running");
        return Ok(());
    }

    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable");
    let elephantry = elephantry::Pool::new(&database_url).expect("Unable to connect to postgresql");

    let sources = elephantry.find_all::<SourceModel>(None)?;

    for source in sources {
        log::info!("Fetching {}", source.url);

        let contents = attohttpc::get(&source.url).send()?.text()?;
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
    }

    Ok(())
}

fn icon(links: &[feed_rs::model::Link]) -> Option<String> {
    let selector = scraper::Selector::parse("link[rel=\"icon\"]").unwrap();

    for link in links {
        let contents = match attohttpc::get(&link.href).send() {
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
