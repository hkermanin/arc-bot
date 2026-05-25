use crate::bot::state::State;
use crate::bot::types::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

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
            
            if let Some(msg) = q.message {
                bot.send_message(msg.chat().id, "some").await?;
            }
        }
    }

    Ok(())
}
