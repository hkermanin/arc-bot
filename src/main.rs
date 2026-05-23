use std::sync::Arc;
use tokio::sync::Mutex;

mod bot;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let wallets: bot::types::Wallets = Arc::new(Mutex::new(vec![]));

    bot::run_bot(wallets).await;
}
