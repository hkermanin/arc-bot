use crate::bot::keyboards::menu_keyboard;
use teloxide::prelude::*;
use crate::bot::types::{MyDialogue,Wallets,HandlerResult};
use crate::bot::state::State;

pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
    dialogue: MyDialogue,
    wallets: Wallets,
) -> HandlerResult {
    if let Some(data) = q.data {
        bot.answer_callback_query(q.id).await?;

        if data == "add" {
            dialogue.update(State::WaitingAdd).await?;

            if let Some(msg) = q.message {
                bot.send_message(msg.chat().id, "Please enter your task:")
                    .await?;
            }
        } else if data == "list" {
            if let Some(msg) = q.message {
                let wallets = wallets.lock().await;
                let text = wallets.join("\n");
                bot.send_message(msg.chat().id, "List of your task:")
                    .reply_markup(menu_keyboard())
                    .await?;
                bot.send_message(msg.chat().id, text).await?;
            }
        }
    }

    Ok(())
}
