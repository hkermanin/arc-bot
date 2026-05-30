use crate::arc::wallet::init_arc_wallet;

mod arc;
mod bot;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    pretty_env_logger::init();

    // init_arc_wallet().await?;

    let db = db::init_db().await?;

    bot::run_bot(db).await;

    Ok(())
}
