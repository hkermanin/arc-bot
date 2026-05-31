use crate::arc::wallet::init::WalletConfig;
use crate::bot::state::State;
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
    }

    Ok(())
}
