use crate::arc::wallet::init::WalletConfig;
use crate::bot::keyboards::{cancel_confirm_send_keyboard, cancel_send_keyboard};
use crate::bot::state::{SendState, State};
use crate::bot::types::{HandlerResult, MyDialogue};
use teloxide::prelude::*;
use teloxide::types::MessageId;
use teloxide::types::ParseMode;

pub async fn send_1(
    prompt_message_id: MessageId,
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> HandlerResult {
    let recipient = msg.text().unwrap().to_string();

    bot.delete_message(msg.chat.id, msg.id).await?;

    bot.edit_message_text(msg.chat.id, prompt_message_id, "💰 Enter Amount:")
        .reply_markup(cancel_send_keyboard())
        .await?;

    dialogue
        .update(State::Send(SendState::WaitingAmount {
            recipient,
            prompt_message_id,
        }))
        .await?;

    Ok(())
}

pub async fn send_2(
    recipient: String,
    prompt_message_id: MessageId,
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> HandlerResult {
    let amount = msg.text().unwrap().to_string();

    bot.delete_message(msg.chat.id, msg.id).await?;

    // bot.edit_message_text(msg.chat.id, prompt_message_id, "💰 Enter Amount:")
    //     .reply_markup(cancel_send_keyboard())
    //     .await?;

    bot.edit_message_text(
        msg.chat.id,
        prompt_message_id,
        format!(
            "📤 <b>Confirm Transfer</b>\n\n\
         <b>Recipient:</b>\n\
         <code>{}</code>\n\n\
         <b>Amount:</b>\n\
         {}\n\n\
         Please review the details carefully before confirming.",
            recipient, amount,
        ),
    )
    .parse_mode(ParseMode::Html)
    .reply_markup(cancel_confirm_send_keyboard())
    .await?;

    dialogue
        .update(State::Send(SendState::WaitingConfirmation {
            recipient,
            amount,
            prompt_message_id,
        }))
        .await?;

    Ok(())
}

