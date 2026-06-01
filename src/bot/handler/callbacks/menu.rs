use teloxide::prelude::*;

use crate::bot::{
    keyboards::{main_menu_keyboard, wallet_menu_keyboard},
    state::State,
    types::{HandlerResult, MyDialogue},
};

pub async fn wallet_show_bot(bot: Bot, q: CallbackQuery, dialogue: MyDialogue) -> HandlerResult {
    dialogue.update(State::Wallet).await?;
    let text = "\
👛 Wallet Management

Manage your Arc wallet and assets.

Choose one of the options below.";

    bot.edit_message_text(
        q.message.as_ref().unwrap().chat().id,
        q.message.as_ref().unwrap().id(),
        text,
    )
    .reply_markup(wallet_menu_keyboard())
    .await?;

    Ok(())
}

pub async fn back_wallet_show_bot(
    bot: Bot,
    q: CallbackQuery,
    dialogue: MyDialogue,
) -> HandlerResult {
    dialogue.update(State::Start).await?;

    let text = "\
🚀 Arc Dashboard

Welcome to Arc.

Manage your wallet, trade assets, and access AI-powered blockchain insights from one place.

Select an option below to continue.";

    bot.edit_message_text(
        q.message.as_ref().unwrap().chat().id,
        q.message.as_ref().unwrap().id(),
        text,
    )
    .reply_markup(main_menu_keyboard())
    .await?;

    Ok(())
}
