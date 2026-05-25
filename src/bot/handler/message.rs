use crate::bot::keyboards::menu_keyboard;
use crate::bot::state::State;
use crate::bot::types::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    state: State,
    db: sqlx::Pool<sqlx::Postgres>,
) -> HandlerResult {
    match state {
        State::Start => {
            bot.send_message(msg.chat.id, "Hello\nWelcome to ToDo bot.")
                .reply_markup(menu_keyboard())
                .await?;
        }
        State::WaitingAdd => {
            if let Some(text) = msg.text() {
                if let Some(user) = msg.from.as_ref() {
                    sqlx::query(
                        "
                INSERT INTO todos (user_id, text)
                VALUES ($1, $2)
                ",
                    )
                    .bind(user.id.0 as i64)
                    .bind(text)
                    .execute(&db)
                    .await?;

                    dialogue.update(State::Start).await?;

                    bot.send_message(msg.chat.id, format!("{} added to list", text))
                        .reply_markup(menu_keyboard())
                        .await?;
                }
            }
        }
    }

    Ok(())
}
