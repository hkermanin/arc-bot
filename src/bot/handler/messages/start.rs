use crate::bot::keyboards::menu_keyboard;
use crate::bot::state::State;
use crate::bot::types::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

pub async fn start(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    state: State,
    db: sqlx::Pool<sqlx::Postgres>,
) -> HandlerResult {
    bot.send_message(msg.chat.id, "Hello\nWelcome to ToDo bot.")
        .reply_markup(menu_keyboard())
        .await?;

    Ok(())
}
