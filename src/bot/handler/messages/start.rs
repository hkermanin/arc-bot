use crate::bot::keyboards::{create_wallet_keyboard, main_menu_keyboard};
use crate::bot::state::State;
use crate::bot::types::{HandlerResult, MyDialogue};
use crate::db::fun::wallet::find_user;
use teloxide::prelude::*;

pub async fn start(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    state: State,
    db: sqlx::Pool<sqlx::Postgres>,
) -> HandlerResult {
    match find_user(msg.from.unwrap().id.0 as i64, &db).await? {
        Some(user) => {
            bot.delete_message(msg.chat.id, msg.id).await?;

            let text = "\
🚀 Arc Dashboard

Welcome to Arc.

Manage your wallet, trade assets, and access AI-powered blockchain insights from one place.

Select an option below to continue.";

            bot.send_message(msg.chat.id, text)
                .reply_markup(main_menu_keyboard())
                .await?;
        }

        None => {
            bot.delete_message(msg.chat.id, msg.id).await?;

            let text = "\
👋 Welcome to Arc

You don't have a wallet yet.

Create your wallet to start trading and using AI analysis tools.

👇 Click the button below to continue.";

            bot.send_message(msg.chat.id, text)
                .reply_markup(create_wallet_keyboard())
                .await?;
        }
    }

    Ok(())
}
