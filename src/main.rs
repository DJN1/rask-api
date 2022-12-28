extern crate dotenvy;
use actix_cors::Cors;
use actix_session::{
    config::PersistentSession, storage::RedisActorSessionStore, SessionMiddleware,
};
use actix_web::http;
use actix_web::middleware::Logger;
use actix_web::{cookie::time::Duration, App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;
use std::env;

mod db;
mod errors;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    db::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST must be set");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set");

    let redis_host = std::env::var("REDIS_HOST").expect("REDIS_HOST must be set");
    let redis_port = std::env::var("REDIS_PORT").expect("REDIS_PORT must be set");
    let ui_url = std::env::var("UI_URL").expect("UI_URL must be set for frontend");
    let private_key = actix_web::cookie::Key::generate();

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(ui_url.as_str())
            .allowed_methods(vec!["DELETE", "GET", "OPTIONS", "POST", "PUT"])
            .allowed_header(http::header::CONTENT_TYPE);

        App::new()
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(format!("{}:{}", redis_host, redis_port)),
                    private_key.clone(),
                )
                .cookie_name(String::from("session"))
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::hours(3)))
                .cookie_secure(false)
                .build(),
            )
            .configure(routes::user::route::init)
            .configure(routes::auth::route::init)
            .configure(routes::task::route::init)
            .wrap(Logger::default())
    })
    .bind(format!("{}:{}", app_host, app_port))?;
    eprintln!("Server running on http://{}:{}", app_host, app_port);

    return server.run().await;
}
