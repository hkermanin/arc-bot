use crate::arc::wallet::init::WalletConfig;
use crate::bot::handler::messages::send::{send_1, send_2};
use crate::bot::state::{SendState, State};
use crate::bot::types::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

use crate::bot::handler::messages::menu;
use crate::bot::handler::messages::start;

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    state: State,
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> HandlerResult {
    match state {
        State::Start => {
            start::start(bot, msg, dialogue, state, db).await?;
        }
        State::Main => {
            menu::back_menu(bot, msg, dialogue, state, db, wallet_config).await?;
        }
        State::Wallet => {
            menu::back_menu(bot, msg, dialogue, state, db, wallet_config).await?;
        }
        State::Send(SendState) => match SendState {
            SendState::WaitingRecipient { prompt_message_id } => {
                send_1(prompt_message_id, bot, msg, dialogue, db, wallet_config).await?;
            }
            SendState::WaitingAmount {
                recipient,
                prompt_message_id,
            } => {
                send_2(
                    recipient,
                    prompt_message_id,
                    bot,
                    msg,
                    dialogue,
                    db,
                    wallet_config,
                )
                .await?;
            },
            SendState::WaitingConfirmation {
                recipient,
                amount,
                prompt_message_id,
            } => {
                send_2(
                    recipient,
                    prompt_message_id,
                    bot,
                    msg,
                    dialogue,
                    db,
                    wallet_config,
                )
                .await?;
            }
        },
    }

    Ok(())
}
