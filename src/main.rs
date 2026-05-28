mod bot;
mod db;
mod arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    pretty_env_logger::init();

    let db = db::init_db().await?;

    bot::run_bot(db).await;

    Ok(())
}
