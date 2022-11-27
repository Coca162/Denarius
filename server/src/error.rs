#![allow(clippy::module_name_repetitions)]


use axum::{
    http::StatusCode,
    response::{IntoResponse, Response}
};
use thiserror::Error;
use tracing::{event, span, Level};

use std::result::Result as StdResult;
use sqlx::{Error as SQLxError, error::DatabaseError};
use uuid::Error as UUIDError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error")]
    Database(#[from] SQLxError),

    #[error("Failed at parsing a uuid")]
    UUID(#[from] UUIDError),

    #[error("A error for returning a alternate http response")]
    HttpError(StatusCode, &'static str),

    #[error("A unique value already existed: `{0}`")]
    AlreadyExists(&'static str),

    #[error("Failed at finding this resource: `{0}`")]
    NotFound(&'static str),

    #[error("A error for testing")]
    Test,
}

pub trait UniqueValueError<T> {
    fn or_already_exists(self, value: &'static str) -> Result<T>;
}

impl<T> UniqueValueError<T> for Result<T> {
    fn or_already_exists(self, value_name: &'static str) -> Result<T> {
        let err = match self {
            Ok(res) => return Ok(res),
            Err(Error::Database(db_error)) => db_error,
            Err(err) => return Err(err),
        };

        to_unique_error!(err, value_name)
    }
}

impl<T> UniqueValueError<T> for StdResult<T, SQLxError> {
    fn or_already_exists(self, value_name: &'static str) -> Result<T> {
        let err = match self {
            Ok(res) => return Ok(res),
            Err(err) => err,
        };

        to_unique_error!(err, value_name)
    }
}

macro_rules! to_unique_error {
    ($err:tt, $value_name:tt) => { 
        {
            if let Some(code) = $err.as_database_error().and_then(DatabaseError::code) {
                if code == "23505" {
                    return Err(Error::AlreadyExists($value_name));
                }
            }
            return Err($err.into());    
        }
     }
}

pub(crate) use to_unique_error;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let span = span!(Level::DEBUG, "error_handling");
        let _entered = span.enter();

        match self {
            Error::UUID(error) =>  (StatusCode::BAD_REQUEST, error.to_string()).into_response(),
            Error::NotFound(resource) => {
                (StatusCode::NOT_FOUND, format!("Could not find {}", resource)).into_response()
            }
            Error::AlreadyExists(error) => (StatusCode::BAD_REQUEST, error).into_response(),
            Error::HttpError(status_code, msg) => (status_code, msg).into_response(),
            _ => {
                event!(Level::ERROR, %self, "Received unexpected error");
                (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error has occurred").into_response()
            }
        }
    }
}

pub enum Example<T, B> {
    Condition1(T),
    Condition3(B)
}