use actix_web::{web, web::Data, App, HttpServer};

use deadpool_postgres::Runtime;
use tokio_postgres::NoTls;

use dotenv::dotenv;

use ansi_term::Colour::*;
use env_logger::Env;
use log::*;

mod config;
mod db;
mod errors;
mod handlers;
mod models;

use crate::handlers::*;
use crate::models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("{}", Green.paint("🦀 🏁 Starting up !"));

    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    info!("{} 💼 Extracted config from .env file", Green.paint("[✓]"));
    debug!("\n 💼 Extracted Data: \n {:#?}", config);

    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    info!("{} 🚣 Started postgres connection pool", Green.paint("[✓]"));

    info!(
        "{} 🌐 Attempting to start server on {}",
        Yellow.paint("[-]"),
        Yellow.underline().paint(format!(
            "http://{}:{}",
            config.server.host, config.server.port
        ))
    );

    log_routes();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { pool: pool.clone() }))
            .service(status)
            .service(add_entry)
            .service(register)
            //.service(get_devices)
            //.service(get_entries)
            .default_service(web::route().to(not_found))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
