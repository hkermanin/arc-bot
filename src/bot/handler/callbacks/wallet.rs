use crate::arc::wallet::arc_create_wallet;
use crate::arc::wallet::init::WalletConfig;
use crate::bot::keyboards::main_menu_keyboard;
use crate::bot::state::State;
use crate::bot::types::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

pub async fn create_new_wallet(
    bot: Bot,
    q: CallbackQuery,
    dialogue: MyDialogue,
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> HandlerResult {
    let result = arc_create_wallet(&q.from.id.0, db, wallet_config).await?;
    let result = format!(
        "✅ Wallet Created Successfully\n\n\
    Your wallet has been created and linked to your account.\n\n\
    🔹 Wallet Address:\n\
    `{}`\n\n\
    Choose an action below.",
        result
    );
    bot.edit_message_text(q.from.id, q.message.unwrap().id(), result)
        .reply_markup(main_menu_keyboard())
        .await?;
    dialogue.update(State::Main).await?;

    Ok(())
}
