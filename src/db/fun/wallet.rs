use crate::{arc::wallet::wallet::Wallet, bot::types::User};

pub async fn add_user(
    user_id: i64,
    wallet_info: Wallet,
    db: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO users(USER_ID, WALLET_ID, WALLET_ADDRESS) 
        VALUES($1, $2, $3)",
    )
    .bind(user_id)
    .bind(wallet_info.id)
    .bind(wallet_info.address)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn find_user(
    user_id: i64,
    db: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as("SELECT user_id FROM users WHERE user_id = $1")
        .bind(user_id)
        .fetch_optional(db)
        .await?;

    Ok(user)
}
