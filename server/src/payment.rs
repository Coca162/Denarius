use axum::http::StatusCode;
use uuid::Uuid;
use sqlx::PgPool;
use crate::error::Result;
use crate::error::Error;

pub async fn payment(
    to: Uuid,
    from: Uuid,
    amount: i64,
    force: Option<bool>,
    db: &PgPool,
) -> Result<()> {
    if force != Option::Some(true) {
        #[allow(clippy::comparison_chain)]
        if amount == 0 {
            return Err(Error::HttpError(StatusCode::FORBIDDEN, "Cannot send zero money!"));
        } else if amount < 0 {
            return Err(Error::HttpError(StatusCode::FORBIDDEN, "You cannot pay negative money"));
        }
    }

    let mut tx = db.begin().await?;

    let balance = sqlx::query!("SELECT balance FROM account WHERE id = $1", from)
        .fetch_one(&mut tx)
        .await?
        .balance;

    if balance < amount {
        return Err(Error::HttpError(StatusCode::BAD_REQUEST, "You lack the funds to send this payment"));
    };

    sqlx::query!(
        "UPDATE account SET balance = balance - $1 WHERE id = $2",
        amount,
        from
    )
    .execute(&mut tx)
    .await?;

    sqlx::query!(
        "UPDATE account SET balance = balance + $1 WHERE id = $2",
        amount,
        to
    )
    .execute(&mut tx)
    .await?;

    let transaction_uuid = Uuid::now_v7();

    sqlx::query!(
        "INSERT INTO transaction_log (id, from_id, to_id, amount) VALUES ($1, $2, $3, $4)",
        transaction_uuid,
        from,
        to,
        amount
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
}