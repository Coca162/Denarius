use axum::{
    Router, 
    routing::{get, post},
    extract::{Path, State},
    http::StatusCode,
    Json
};
use shared::types::PersonInfo;
use sqlx::PgPool;
use uuid::Uuid;

use crate::discord_id::DiscordId;
use crate::error::Error;
use crate::error::Result;
use crate::error::UniqueValueError;

#[must_use]
pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/register/:discord_id", post(register))
        .route("/from_discord/:discord_id", get(get_from_discord))
        .route("/:id", get(get_from_uuid))
}

pub async fn register(Path(discord_id): Path<u64>, State(db): State<PgPool>) -> Result<(StatusCode, String)> {
    let mut tx = db.begin().await?;

    let uuid = Uuid::now_v7();

    sqlx::query!("INSERT INTO account (id) VALUES ($1)", &uuid)
        .execute(&mut tx)
        .await?;

    sqlx::query!(
        "INSERT INTO person (id, discord_id) VALUES ($1, $2)",
        &uuid,
        &*DiscordId::from(discord_id)
    )
    .execute(&mut tx)
    .await
    .or_already_exists("This person is already registered!")?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, uuid.as_simple().to_string()))
}

pub async fn get_from_discord(Path(discord_id): Path<u64>, State(db): State<PgPool>) -> Result<String> {
    let result = sqlx::query!("SELECT id FROM person WHERE person.discord_id = $1", &*DiscordId::from(discord_id))
                            .fetch_optional(&db)
                            .await?
                            .ok_or(Error::NotFound("person"))?.id;

    Ok(result.as_simple().to_string())
}

pub async fn get_from_uuid(Path(id): Path<Uuid>, State(db): State<PgPool>) -> Result<Json<PersonInfo>> {
    let result = sqlx::query!("SELECT discord_id, balance FROM person INNER JOIN account ON account.id=person.id WHERE person.id = $1", id)
                            .fetch_optional(&db)
                            .await?
                            .ok_or(Error::NotFound("person"))?;

    let result = PersonInfo {
        discord_id: DiscordId::from(result.discord_id).into(),
        balance: result.balance,
    };

    Ok(Json(result))
}