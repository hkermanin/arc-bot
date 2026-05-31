use crate::arc::wallet::init::WalletConfig;
use crate::bot::handler::callbacks::menu::{back_wallet_show_bot, wallet_show_bot};
use crate::bot::handler::callbacks::wallet::create_new_wallet;
use crate::bot::keyboards::{main_menu_keyboard, wallet_menu_keyboard};
use crate::bot::state::State;
use crate::bot::types::{HandlerResult, MyDialogue};
use teloxide::prelude::*;


pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
    dialogue: MyDialogue,
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> HandlerResult {
    if let Some(data) = &q.data {
        bot.answer_callback_query(q.id.clone()).await?;

        if data == "wallet" {
           wallet_show_bot(bot, q, dialogue).await?;
        } else if data == "back_wallet" {
            back_wallet_show_bot(bot, q, dialogue).await?;
          
        } else if data == "new_wallet" {
            create_new_wallet(bot, q, dialogue, db, wallet_config).await?;
        }
    }

    Ok(())
}
