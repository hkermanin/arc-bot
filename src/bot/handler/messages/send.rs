use crate::arc::wallet::init::WalletConfig;
use crate::bot::keyboards::main_menu_keyboard;
use crate::bot::state::State;
use crate::bot::types::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

pub async fn send(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    state: State,
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> HandlerResult {
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
                .reply_markup(main_menu_keyboard())
                .await?;
        }
    }

    Ok(())
}
