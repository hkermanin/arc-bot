use sqlx::PgPool;

pub mod fun;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let db = PgPool::connect(&database_url).await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS todos (
            id SERIAL PRIMARY KEY,
            user_id BIGINT NOT NULL,
            text TEXT NOT NULL
        );
        ",
    )
    .execute(&db)
    .await?;

    Ok(db)
}
