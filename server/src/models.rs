use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub response: String,
}

#[derive(Debug, Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "devices")]
pub struct Device {
    pub id: String,
    pub key: String,
    pub dev_location: String,
    pub created_at: String,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct NewDevice {
    #[validate(length(min = 1, max = 500))]
    pub dev_location: String,
}

#[derive(Debug, Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "entries")]
pub struct Entry {
    pub id: i32,
    pub device_id: String,
    pub val: i16,
    pub created_at: String,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct NewEntry {
    #[validate(length(equal = 128))]
    pub device_id: String,
    #[validate(range(min = -1000, max = 1000))]
    pub val: i16,
    #[validate(length(equal = 512))]
    pub key: String,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}
