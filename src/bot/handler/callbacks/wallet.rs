use crate::arc::wallet::arc_create_wallet;
use crate::arc::wallet::balance::show_balance;
use crate::arc::wallet::init::WalletConfig;
use crate::arc::wallet::send::send_transaction;
use crate::bot::handler::callbacks::menu::{back_wallet_show_bot, wallet_show_bot};
use crate::bot::keyboards::{cancel_send_keyboard, main_menu_keyboard, wallet_menu_keyboard};
use crate::bot::state::{SendState, State};
use crate::bot::types::{HandlerResult, MyDialogue};
use image::{DynamicImage, ImageFormat, Luma};
use qrcode::QrCode;
use std::io::Cursor;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile, ParseMode};

pub async fn send_to(bot: Bot, q: CallbackQuery, dialogue: MyDialogue) -> HandlerResult {
    let msg = bot
        .edit_message_text(
            q.message.as_ref().unwrap().chat().id,
            q.message.as_ref().unwrap().id(),
            "📤 Send USDC\n\nPlease enter the recipient wallet address:",
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

pub async fn receive_assets(
    bot: Bot,
    q: CallbackQuery,
    dialogue: MyDialogue,
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> HandlerResult {
    let address: String = sqlx::query_scalar("SELECT wallet_address FROM users WHERE user_id = $1")
        .bind(q.from.id.0 as i64)
        .fetch_one(&db)
        .await?;

    let text = format!(
        "📥 Receive Assets\n\n\
     Send assets to the wallet address below:\n\n\
     <code>{}</code>\n\n\
     ⚠️ Only send assets supported on ARC Testnet.",
        address
    );

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("🔳 QR Code", "qr_code")],
        vec![InlineKeyboardButton::callback("⬅️ Back", "cancel_send")],
    ]);
    bot.edit_message_text(q.from.id, q.message.unwrap().id(), text)
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(State::Main).await?;

    Ok(())
}

pub async fn qr_code_generator(
    bot: Bot,
    q: CallbackQuery,
    dialogue: MyDialogue,
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> HandlerResult {
    let address: String = sqlx::query_scalar("SELECT wallet_address FROM users WHERE user_id = $1")
        .bind(q.from.id.0 as i64)
        .fetch_one(&db)
        .await?;

    let code = QrCode::new(&address)?;

    let image = code.render::<Luma<u8>>().build();

    let mut bytes = Vec::new();

    DynamicImage::ImageLuma8(image).write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)?;

    let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "🗑 Close",
        "close_qr",
    )]]);

    bot.send_photo(
        q.from.id,
        InputFile::memory(bytes).file_name("wallet_qr.png"),
    )
    .caption(format!("📥 Wallet QR Code\n\n<code>{}</code>", address))
    .parse_mode(ParseMode::Html)
    .reply_markup(keyboard)
    .await?;

    dialogue.update(State::Main).await?;

    Ok(())
}

pub async fn close_qr_bot(bot: Bot, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(q.id.clone()).await?;

    if let Some(message) = &q.message {
        bot.delete_message(q.from.id, message.id()).await?;
    }

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
    <code>{}</code>\n\n\
    Choose an action below.",
        result
    );
    bot.edit_message_text(q.from.id, q.message.unwrap().id(), result)
        .reply_markup(main_menu_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    dialogue.update(State::Main).await?;

    Ok(())
}
