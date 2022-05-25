use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use ansi_term::Colour::*;
use deadpool_postgres::PoolError;
use log::*;
use serde::Serialize;
use std::fmt;
use tokio_postgres::error::Error;

#[derive(Debug)]
pub enum AppErrorType {
    DBError,
    NotFoundError,
    AuthError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                ..
            } => message.clone(),
            AppError {
                message: None,
                error_type: AppErrorType::NotFoundError,
                ..
            } => "The requested item was not found".to_string(),
            AppError {
                message: None,
                error_type: AppErrorType::AuthError,
                ..
            } => "You do not have authorization to run this request".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl From<PoolError> for AppError {
    fn from(error: PoolError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DBError,
        }
    }
}

impl From<Error> for AppError {
    fn from(error: Error) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DBError,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DBError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::AuthError => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> HttpResponse {
        info!(
            "💀 {} {} {:?}",
            Red.paint("ERROR"),
            Yellow.paint(self.status_code().as_str()),
            self.message
        );
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}
