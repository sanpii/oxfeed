mod cache;
mod form;
mod identity;
mod services;

use identity::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable");
    let ip = std::env::var("LISTEN_IP").expect("Missing LISTEN_IP env variable");
    let port = std::env::var("LISTEN_PORT").expect("Missing LISTEN_IP env variable");
    let bind = format!("{}:{}", ip, port);

    actix_web::HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();

        let elephantry =
            elephantry::Pool::new(&database_url).expect("Unable to connect to postgresql");

        actix_web::App::new()
            .wrap(actix_web::middleware::NormalizePath::new(
                actix_web::middleware::normalize::TrailingSlash::Trim,
            ))
            .data(elephantry)
            .wrap(cors)
            .service(services::auth::scope())
            .service(services::icon::scope())
            .service(services::item::scope())
            .service(services::opml::scope())
            .service(services::search::scope())
            .service(services::source::scope())
            .service(services::user::scope())
            .service(services::websocket::scope())
            .service(services::scope())
    })
    .bind(&bind)?
    .run()
    .await
}
