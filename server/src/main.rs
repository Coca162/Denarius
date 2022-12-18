#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use axum::{
    routing::get,
    extract::State,
    Router,
};
use std::net::SocketAddr;
use dotenvy::dotenv;
use std::env;
use sqlx::{postgres::PgPoolOptions, PgPool};
//use log::{log, debug, info, warn, error, trace};

#[allow(clippy::wildcard_imports)]
use crate::routes::*;
use crate::error::Error;
use crate::error::Result;

pub mod discord_id;
pub mod error;
pub mod routes;
pub mod timestamp;
pub mod payment;

#[tokio::main]
pub async fn main() {
    dotenv().expect("A .env file does not exist!");
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .min_connections(2)
        .connect(&env::var("DATABASE_URL").expect("Could not find database url from environment variables!"))
        .await
        .expect("Failed to connect to database");

    let app = routes().with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080)); //127.0.0.1:8080
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn routes() -> Router<PgPool> {
    Router::new()
            .route("/", get(index))
            .route("/yeet_error", get(yeet_error))
            .nest("/person", person::routes())
            .nest("/eco", eco::routes())
}

async fn index(State(db): State<PgPool>) -> Result<String> {
    let result = sqlx::query!("SELECT COUNT(*) as \"count!\" FROM account")
        .fetch_one(&db)
        .await?
        .count;

    Ok(format!("Hello, world! Accounts: {result}"))
}

#[allow(clippy::unused_async)]
async fn yeet_error() -> Result<String> {
    Err(Error::Test)
}
