use crate::bot::state::State;
use crate::bot::types::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

#[derive(sqlx::FromRow)]
struct Todo {
    text: String,
}

pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
    dialogue: MyDialogue,
    db: sqlx::Pool<sqlx::Sqlite>,
) -> HandlerResult {
    if let Some(data) = q.data {
        bot.answer_callback_query(q.id).await?;

        if data == "add" {
            dialogue.update(State::WaitingAdd).await?;

            if let Some(msg) = q.message {
                bot.send_message(msg.chat().id, "Please enter your task:")
                    .await?;
            }
        } else if data == "list" {
            dialogue.update(State::Start).await?;

            let todos = sqlx::query_as::<_, Todo>(
                "
                SELECT text FROM todos
                WHERE user_id = ?
            ",
            )
            .bind(q.from.id.0 as i64)
            .fetch_all(&db)
            .await?;

            let text = todos
                .iter()
                .enumerate()
                .map(|(i, todo)| format!("{}. {}", i + 1, todo.text))
                .collect::<Vec<_>>()
                .join("\n");

            if let Some(msg) = q.message {
                bot.send_message(msg.chat().id, text).await?;
            }
        }
    }

    Ok(())
}
