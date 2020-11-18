use oxfeed_api::model::item::Model as ItemModel;
use oxfeed_api::model::source::Model as SourceModel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                let item = oxfeed_api::model::item::Entity {
                    entry_id: None,
                    id: entry.id,
                    icon: None,
                    content: entry.content.map(|x| x.body).flatten(),
                    title: entry.title.map(|x| x.content).unwrap_or_default(),
                    published: entry.published,
                    read: false,
                    source_id: source.source_id.unwrap(),
                    link: entry.links[0].href.clone(),
                };
                elephantry.insert_one::<ItemModel>(&item)?;
            }
        }
    }

    Ok(())
}
