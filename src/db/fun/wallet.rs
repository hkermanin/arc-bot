use crate::bot::types::User;

pub async fn find_user(user_id: i64, db: &sqlx::Pool<sqlx::Postgres>) -> Result<Option<User>, sqlx::Error>{

    let user = sqlx::query_as("SELECT user_id FROM todos WHERE user_id = $1")
        .bind(user_id)
        .fetch_optional(db)
        .await?;

    Ok(user)

}