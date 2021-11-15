#![warn(warnings)]

mod cache;
mod form;
mod identity;
mod services;
mod update;

use identity::*;

#[actix_web::main]
async fn main() -> oxfeed_common::Result {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    env_logger::init();

    let database_url = env("DATABASE_URL")?;
    let ip = env("LISTEN_IP")?;
    let port = env("LISTEN_PORT")?;
    let bind = format!("{}:{}", ip, port);

    let elephantry = elephantry::Pool::new(&database_url)?;

    let update = update::Actor::new(&elephantry);
    let actor = actix_web::web::Data::new(update.start());

    actix_web::HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();

        actix_web::App::new()
            .wrap(actix_web::middleware::NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
            .app_data(actix_web::web::Data::new(actor.clone()))
            .app_data(actix_web::web::Data::new(elephantry.clone()))
            .wrap(cors)
            .service(services::auth::scope())
            .service(services::icon::scope())
            .service(services::item::scope())
            .service(services::opml::scope())
            .service(services::search::scope())
            .service(services::source::scope())
            .service(services::tag::scope())
            .service(services::user::scope())
            .service(services::webhook::scope())
            .service(services::websocket::scope())
            .service(services::scope())
    })
    .bind(&bind)?
    .run()
    .await?;

    Ok(())
}

fn env(name: &str) -> oxfeed_common::Result<String> {
    std::env::var(name).map_err(|_| oxfeed_common::Error::Env(name.to_string()))
}
