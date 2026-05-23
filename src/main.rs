use teloxide::{
    dispatching::{
        UpdateFilterExt,
        dialogue::{Dialogue, InMemStorage},
    },
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
enum State {
    #[default]
    Start,

    WaitingAdd,

}

type Wallets = Arc<Mutex<Vec<String>>>;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let wallets: Wallets = Arc::new(Mutex::new(vec![]));

    let bot = Bot::from_env();

    let storage = InMemStorage::<State>::new();
    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<State>, State>()
                .endpoint(message_handler),
        )
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<CallbackQuery, InMemStorage<State>, State>()
                .endpoint(callback_handler),
        );
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage, wallets])
        .build()
        .dispatch()
        .await;
}

fn menu_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback("Add", "add"),
        InlineKeyboardButton::callback("List", "list"),
    ]])
}

async fn message_handler(
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
                        .reply_markup(menu_keyboard()).await?;

                    dialogue.update(State::Start).await?;
            }
        }
        
    }

    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery, dialogue: MyDialogue, wallets: Wallets,) -> HandlerResult {
    if let Some(data) = q.data {
        bot.answer_callback_query(q.id).await?;

        if data == "add" {
            dialogue.update(State::WaitingAdd).await?;

            if let Some(msg) = q.message {
                bot.send_message(msg.chat().id, "Please enter your task:").await?;
            }
        }else if data == "list"{
            if let Some(msg) = q.message {
                let wallets = wallets.lock().await;
                let text = wallets.join("\n");
                bot.send_message(msg.chat().id, "List of your task:")
                .reply_markup(menu_keyboard()).await?;
                bot.send_message(msg.chat().id, text).await?;
            }
        }
    }

    Ok(())
}
