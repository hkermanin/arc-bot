use crate::bot::keyboards::{create_wallet_keyboard, menu_keyboard};
use crate::bot::state::State;
use crate::bot::types::{HandlerResult, MyDialogue};
use crate::db::fun::wallet::find_user;
use teloxide::prelude::*;

pub async fn start(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    state: State,
    db: sqlx::Pool<sqlx::Postgres>,
) -> HandlerResult {
    match find_user(msg.from.unwrap().id.0 as i64, &db).await?{
    
        Some(user) => {
     
            bot.send_message(msg.chat.id, "You Have wallet")
                .reply_markup(menu_keyboard())
                .await?;
        
        },


        None => {
            
            let text = "\
     Welcome to the Arc Dex trading Bot!
To get started, you first need to create your wallet.
";
            bot.send_message(msg.chat.id, text)
                .reply_markup(create_wallet_keyboard())
                .await?;
        
        }

    
    }
    

    Ok(())
}
