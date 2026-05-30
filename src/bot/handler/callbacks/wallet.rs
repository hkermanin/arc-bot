use crate::arc::wallet::arc_create_wallet;
use crate::arc::wallet::init::WalletConfig;
use crate::bot::types::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

pub async fn create_new_wallet(
    bot: Bot,
    q: CallbackQuery,
    dialogue: MyDialogue,
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> HandlerResult {
    let result = arc_create_wallet().await;
    let result = format!("Your wallet created: {}", result);
    if let Some(msg) = q.message {
        bot.send_message(msg.chat().id, result).await?;
    }

    Ok(())
}
