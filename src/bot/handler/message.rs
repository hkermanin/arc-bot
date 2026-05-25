use crate::bot::keyboards::menu_keyboard;
use crate::bot::state::State;
use crate::bot::types::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    state: State,
    db: sqlx::Pool<sqlx::Sqlite>,
) -> HandlerResult {
    match state {
        State::Start => {
            bot.send_message(msg.chat.id, "Hello\nWlecom to my bot.")
                .reply_markup(menu_keyboard())
                .await?;
        }
        State::WaitingAdd => {
            if let Some(text) = msg.text() {
                bot.send_message(msg.chat.id, format!("Added task: {}", text))
                    .reply_markup(menu_keyboard())
                    .await?;

                dialogue.update(State::Start).await?;
            }
        }
    }

    Ok(())
}
