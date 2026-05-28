use crate::bot::state::State;
use crate::bot::types::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

use crate::bot::handler::messages::send;
use crate::bot::handler::messages::start;

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    state: State,
    db: sqlx::Pool<sqlx::Postgres>,
) -> HandlerResult {
    match state {
        State::Start => {
            start::start(bot, msg, dialogue, state, db).await?;
        }
        State::Send => {
            send::send(bot, msg, dialogue, state, db).await?;
        }
    }

    Ok(())
}
