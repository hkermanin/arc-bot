use teloxide::{
    dispatching::{UpdateFilterExt, dialogue::InMemStorage},
    prelude::*,
};

mod handler;
mod keyboards;
mod state;
pub mod types;

use handler::callback::callback_handler;
use handler::message::message_handler;
use state::State;

pub async fn run_bot(wallets: types::Wallets) {
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
