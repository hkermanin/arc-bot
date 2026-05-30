use crate::arc::wallet::setwallet::WalletSet;

#[derive(sqlx::FromRow)]
pub struct WalletConfigDB {
    pub wallet_id: String,
    pub wallet_name: String,
}

pub async fn find_wallet_id(
    db: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Option<WalletConfigDB>, sqlx::Error> {
    let wallet_id = sqlx::query_as::<_, WalletConfigDB>("SELECT * FROM walletconfig")
        .fetch_optional(db)
        .await?;

    Ok(wallet_id)
}

pub async fn add_wallet_id(
    wallet_set: WalletSet,
    db: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO walletconfig VALUES ($1, $2)")
        .bind(wallet_set.id)
        .bind(wallet_set.name)
        .execute(db)
        .await?;

    Ok(())
}
