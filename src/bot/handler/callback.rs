use crate::arc::wallet::init::WalletConfig;
use crate::bot::handler::callbacks::wallet::create_new_wallet;
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
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> HandlerResult {
    if let Some(data) = &q.data {
        bot.answer_callback_query(q.id.clone()).await?;

        if data == "add" {
            dialogue.update(State::Send).await?;

            if let Some(msg) = q.message {
                bot.send_message(msg.chat().id, "Please enter your task:")
                    .await?;
            }
        } else if data == "list" {
            dialogue.update(State::Start).await?;

            let todos = sqlx::query_as::<_, Todo>(
                "
                SELECT text FROM todos
                WHERE user_id = $1
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

            if let Some(msg) = &q.message {
                bot.send_message(msg.chat().id, text).await?;
            }
        } else if data == "new_wallet" {
            create_new_wallet(bot, q, dialogue, db, wallet_config).await?;
        }
    }

    Ok(())
}
