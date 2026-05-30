use crate::arc::wallet::init::init_arc_wallet;

mod arc;
mod bot;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    pretty_env_logger::init();

    let db = db::init_db().await?;

    let wallet_config = init_arc_wallet(&db).await?;

    bot::run_bot(db, wallet_config).await;

    Ok(())
}
