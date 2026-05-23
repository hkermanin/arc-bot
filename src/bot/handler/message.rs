use crate::bot::keyboards::menu_keyboard;
use teloxide::prelude::*;
use crate::bot::types::{MyDialogue,Wallets,HandlerResult};
use crate::bot::state::State;

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    state: State,
    wallets: Wallets,
) -> HandlerResult {
    match state {
        State::Start => {
            bot.send_message(msg.chat.id, "Hello\nWlecom to my bot.")
                .reply_markup(menu_keyboard())
                .await?;
        }
        State::WaitingAdd => {
            if let Some(text) = msg.text() {
                {
                    let mut wallets = wallets.lock().await;

                    wallets.push(text.to_string());
                }

                bot.send_message(msg.chat.id, format!("Added task: {}", text))
                    .reply_markup(menu_keyboard())
                    .await?;

                dialogue.update(State::Start).await?;
            }
        }
    }

    Ok(())
}
