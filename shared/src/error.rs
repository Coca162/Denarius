#![allow(clippy::module_name_repetitions)]

use async_trait::async_trait;
use reqwest::{Response, StatusCode};
use thiserror::Error;

use reqwest::Error as ReqwestError;
use std::{num::ParseFloatError, num::ParseIntError, result::Result as StdResult};
use uuid::Error as UUIDError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("There was a error trying to call the api")]
    Reqwest(#[from] ReqwestError),

    #[error("Failed at parsing a account uuid")]
    UUID(#[from] UUIDError),

    #[error("Failed at parsing the balance from a request")]
    BalanceParsing(#[from] ParseIntError),

    #[error("Failed at parsing money from a string")]
    MoneyFromString(#[from] MoneyFromStringError),

    #[error("{code} Response: {message}")]
    FailedResponse { code: StatusCode, message: String },

    #[error("Unknown Error")]
    Unknown,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MoneyFromStringError {
    #[error("Failed at parsing the money from a float")]
    FailedFloatParsing(#[from] ParseFloatError),

    #[error("The value given was too small")]
    ValueTooPrecise,
}

#[async_trait]
pub trait FailedResponseError<T> {
    async fn verify_success(self) -> Result<T>;
}

#[async_trait]
impl FailedResponseError<Response> for Response {
    async fn verify_success(self) -> Result<Response> {
        if !self.status().is_success() {
            return Err(Error::FailedResponse {
                code: self.status(),
                message: self.text().await?,
            });
        }

        Ok(self)
    }
}
