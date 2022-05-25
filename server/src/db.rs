use crate::errors::AppError;
use crate::errors::AppErrorType::*;
use crate::models::{Device, Entry, NewDevice, NewEntry};
use deadpool_postgres::Client;
use passwords::PasswordGenerator;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

pub async fn create_device(client: &Client, new_device: NewDevice) -> Result<Device, AppError> {
    let pgen = PasswordGenerator {
        length: 64,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: true,
        strict: true,
    };

    let new_uuid = Uuid::new_v4().to_string();
    let new_key = pgen.generate_one().unwrap().to_string();

    let statement = client
        .prepare(r#"insert into devices (id, key, dev_location) values ($1, $2, $3) returning id, key, dev_location, TO_CHAR(created_at,'YYYY-MM-DD-HH24.MI.SS.FF6') AS "created_at""#)
        .await?;

    client
        .query(
            &statement,
            &[&new_uuid, &new_key, &new_device.dev_location.to_string()],
        )
        .await?
        .iter()
        .map(|row| Device::from_row_ref(row).unwrap())
        .collect::<Vec<Device>>()
        .pop()
        .ok_or(AppError {
            message: Some("Error creating device".to_string()),
            cause: Some("Unknown error.".to_string()),
            error_type: DBError,
        })
}

pub async fn get_devices(client: &Client) -> Result<Vec<Device>, AppError> {
    let statement = client.prepare(r#"select id, key, dev_location, TO_CHAR(created_at,'YYYY-MM-DD-HH24.MI.SS.FF6') AS "created_at" from devices"#).await?;

    let devices = client
        .query(&statement, &[])
        .await?
        .iter()
        .map(|row| Device::from_row_ref(row).unwrap())
        .collect::<Vec<Device>>();

    Ok(devices)
}

pub async fn create_entry(client: &Client, new_entry: NewEntry) -> Result<Entry, AppError> {
    let statement = client
        .prepare(r#"select id, key, dev_location, TO_CHAR(created_at,'YYYY-MM-DD-HH24.MI.SS.FF6') AS "created_at" from devices where id = $1"#)
        .await?;

    let maybe_device = client
        .query_opt(&statement, &[&new_entry.device_id])
        .await?
        .map(|row| Device::from_row_ref(&row).unwrap());

    if maybe_device.is_none() {
        AppError {
            error_type: NotFoundError,
            cause: None,
            message: Some(format!("Device not found.",)),
        };
    } else {
        let device = maybe_device.unwrap();
        if device.key != new_entry.key {
            AppError {
                error_type: AuthError,
                cause: None,
                message: Some(format!("Device key does not match.",)),
            };
        }
    }

    let statement = client
        .prepare(r#"insert into entries (device_id, val) values ($1, $2) returning id, device_id, val, TO_CHAR(created_at,'YYYY-MM-DD-HH24.MI.SS.FF6') AS "created_at""#)
        .await?;

    client
        .query(&statement, &[&new_entry.device_id, &new_entry.val])
        .await?
        .iter()
        .map(|row| Entry::from_row_ref(row).unwrap())
        .collect::<Vec<Entry>>()
        .pop()
        .ok_or(AppError {
            message: Some("Error creating entry".to_string()),
            cause: Some("Unknown error.".to_string()),
            error_type: DBError,
        })
}

pub async fn get_entries(client: &Client) -> Result<Vec<Entry>, AppError> {
    let statement = client.prepare(r#"select id, device_id, val, TO_CHAR(created_at,'YYYY-MM-DD-HH24.MI.SS.FF6') AS "created_at" from entries"#).await?;

    let entries = client
        .query(&statement, &[])
        .await?
        .iter()
        .map(|row| Entry::from_row_ref(row).unwrap())
        .collect::<Vec<Entry>>();

    Ok(entries)
}
