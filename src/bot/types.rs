// use std::sync::Arc;
// use tokio::sync::Mutex;

use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};

use crate::bot::state::State;

// pub type messages = Arc<Mutex<Vec<String>>>;

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub user_id: i64,
}
