use sqlx::PgPool;

pub mod fun;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let db = PgPool::connect(&database_url).await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            user_id BIGINT NOT NULL UNIQUE,
            wallet_id TEXT NOT NULL UNIQUE,
            wallet_address TEXT NOT NULL UNIQUE
        );
        ",
    )
    .execute(&db)
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS walletconfig (
            wallet_id TEXT PRIMARY KEY,
            wallet_name TEXT NOT NULL
        );
        ",
    )
    .execute(&db)
    .await?;

    Ok(db)
}
