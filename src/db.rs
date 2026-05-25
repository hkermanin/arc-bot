use sqlx::SqlitePool;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let db = SqlitePool::connect("sqlite://database.db").await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            text TEXT NOT NULL
        );
        ",
    )
    .execute(&db)
    .await?;

    Ok(db)
}
