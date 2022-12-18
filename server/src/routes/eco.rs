use axum::{
    Router, 
    routing::{get, post}, 
    extract::{Path, State, Query}
};
use shared::types::PaymentParams;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Result;

#[must_use]
pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/payment", post(payment))
        .route("/print/:id/:amount", post(print_money))
        .route("/balance/:id", get(get_balance))
}

pub async fn payment(
    queries: Query<PaymentParams>,
    State(db): State<PgPool>,
) -> Result<&'static str> {
    crate::payment::payment(queries.to, queries.from, queries.amount, queries.force, &db).await?;

    Ok("Success")
}

pub async fn print_money(
    Path((id, amount)): Path<(Uuid, i64)>,
    State(db): State<PgPool>,
) -> Result<&'static str> {

    sqlx::query!(
        "UPDATE account SET balance = balance + $1 WHERE id = $2",
        amount,
        id
    )
    .execute(&db)
    .await?;

    Ok("Printed money!")
}

pub async fn get_balance(Path(id): Path<Uuid>, State(db): State<PgPool>) -> Result<String> {
    let balance = sqlx::query!("SELECT balance FROM account WHERE id = $1", id)
        .fetch_one(&db)
        .await?
        .balance;

    Ok(balance.to_string())
}
