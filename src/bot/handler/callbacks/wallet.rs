use crate::arc::wallet::arc_create_wallet;
use crate::arc::wallet::balance::show_balance;
use crate::arc::wallet::init::WalletConfig;
use crate::arc::wallet::send::send_transaction;
use crate::bot::handler::callbacks::menu::{back_wallet_show_bot, wallet_show_bot};
use crate::bot::keyboards::{cancel_send_keyboard, main_menu_keyboard, wallet_menu_keyboard};
use crate::bot::state::{SendState, State};
use crate::bot::types::{HandlerResult, MyDialogue};
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn send_to(bot: Bot, q: CallbackQuery, dialogue: MyDialogue) -> HandlerResult {
    let msg = bot
        .edit_message_text(
            q.message.as_ref().unwrap().chat().id,
            q.message.as_ref().unwrap().id(),
            "📤 Send Assets\nPlease enter the recipient wallet address:",
        )
        .reply_markup(cancel_send_keyboard())
        .await?;

    dialogue
        .update(State::Send(SendState::WaitingRecipient {
            prompt_message_id: msg.id,
        }))
        .await?;

    Ok(())
}

pub async fn cancel_send(bot: Bot, q: CallbackQuery, dialogue: MyDialogue) -> HandlerResult {
    dialogue.update(State::Wallet).await?;

    wallet_show_bot(bot, q, dialogue).await?;
    Ok(())
}

pub async fn confirm_send(
    bot: Bot,
    q: CallbackQuery,
    dialogue: MyDialogue,
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> HandlerResult {
    let state = dialogue.get().await?;

    let response = match state {
        Some(State::Send(SendState::WaitingConfirmation {
            recipient, amount, ..
        })) => {
            let user_id = q.from.id.0 as i64;
            let response = send_transaction(user_id, recipient, amount, db, wallet_config).await?;
            response
        }

        _ => "Unexpected state. Please start the send process again.".to_string(),
    };

    dialogue.update(State::Wallet).await?;

    let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "⬅️ Back",
        "cancel_send",
    )]]);

    bot.edit_message_text(
        q.message.as_ref().unwrap().chat().id,
        q.message.as_ref().unwrap().id(),
        response,
        // "✅ Transfer Submitted",
    )
    .reply_markup(keyboard)
    .await?;

    Ok(())
}

pub async fn balance_show_bot(
    bot: Bot,
    q: CallbackQuery,
    dialogue: MyDialogue,
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> HandlerResult {
    let result = show_balance(q.from.id.0 as i64, &db, &wallet_config).await?;
    
    let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "⬅️ Back",
        "cancel_send",
    )]]);
    bot.edit_message_text(q.from.id, q.message.unwrap().id(), result)
        .reply_markup(keyboard)
        .await?;
    dialogue.update(State::Main).await?;

    Ok(())
}

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
