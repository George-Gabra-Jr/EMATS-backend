use actix_web::{get, post, web, HttpResponse, Responder};

use deadpool_postgres::{Client, Pool, PoolError};

use ansi_term::Colour::*;
use log::*;

use crate::db;
use crate::errors::AppError;
use crate::models::*;

async fn get_client(pool: Pool) -> Result<Client, AppError> {
    pool.get()
        .await
        .map_err(|err: PoolError| AppError::from(err))
}

#[post("/api/register")]
pub async fn register(
    state: web::Data<AppState>,
    new_device: web::Json<NewDevice>,
) -> Result<impl Responder, AppError> {
    info!("🪃  {} /register", White.paint("POST"));
    let client: Client = get_client(state.pool.clone()).await?;
    let result = db::create_device(&client, new_device.0).await;
    result.map(|devices| HttpResponse::Ok().json(devices))
}

#[post("/api/add_entry")]
pub async fn add_entry(
    state: web::Data<AppState>,
    new_entry: web::Json<NewEntry>,
) -> Result<impl Responder, AppError> {
    info!("🪃  {} /add_entry", White.paint("POST"));
    let client: Client = get_client(state.pool.clone()).await?;
    let result = db::create_entry(&client, new_entry.0).await;
    result.map(|devices| HttpResponse::Ok().json(devices))
}

#[get("/api/devices")]
pub async fn get_devices(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    info!("🪃  {} /get_devices", White.paint("GET"));
    let client: Client = get_client(state.pool.clone()).await?;
    let result = db::get_devices(&client).await;
    result.map(|devices| HttpResponse::Ok().json(devices))
}

#[get("/api/entries")]
pub async fn get_entries(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    info!("🪃  {} /entries", White.paint("GET"));
    let client: Client = get_client(state.pool.clone()).await?;
    let result = db::get_entries(&client).await;
    result.map(|entries| HttpResponse::Ok().json(entries))
}

#[get("/api/status")]
pub async fn status() -> impl Responder {
    info!("🪃  {} /", White.paint("GET"),);
    web::HttpResponse::Ok().json(Response {
        response: "SERVER IS UP ⬆️ AND RUNNING 🏃‍♂️ Hello there 👋 !".to_string(),
    })
}

pub async fn not_found() -> impl Responder {
    info!(
        "💀 {} {} {}",
        Red.paint("ERROR"),
        Yellow.paint("404"),
        "Route not found"
    );
    web::HttpResponse::Ok().json(Response {
        response: "ERROR 404 ! Route not found".to_string(),
    })
}

pub fn log_routes() {
    debug!(
        "\n 🧭 Available routes:\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
        format!(
            "{} \t {} \t \t -> Status",
            White.paint("GET"),
            Yellow.paint("/api/status")
        ),
        format!(
            "{} \t {} \t -> Register device with location to get id and key",
            White.paint("POST"),
            Yellow.paint("/api/register")
        ),
        format!(
            "{} \t {} \t -> Add entry with device id and key",
            White.paint("POST"),
            Yellow.paint("/api/add_entry")
        ),
        format!(
            "{} \t {} \t -> Get all registered devices",
            White.paint("GET"),
            Yellow.paint("/api/devices")
        ),
        format!(
            "{} \t {} \t -> Get all entries",
            White.paint("GET"),
            Yellow.paint("/api/entries")
        ),
    );
}
