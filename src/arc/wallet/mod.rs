use anyhow::Result;
use reqwest::Client;

use crate::arc::wallet::config::arc_config;
use crate::arc::wallet::setwallet::create_set_wallet;
use crate::arc::wallet::wallet::create_wallet;

mod config;
mod encrypt;
mod setwallet;
mod wallet;

pub async fn init_arc_wallet() -> Result<()> {
    let client = Client::new();

    let config = arc_config(&client).await?;

    let wallet_set = create_set_wallet("wallet-set", &client, &config).await?;

    println!(
        "wallet_set_id: {}\nwallet_set_name: {}",
        wallet_set.id, wallet_set.name
    );

    let wallet = create_wallet(&wallet_set.id, &client, &config).await?;

    println!("wallet_id: {}", wallet.id);
    println!("wallet_address: {}", wallet.address);
    println!("blockchain: {}", wallet.blockchain);

    Ok(())
}


pub async fn arc_create_wallet() -> i64 {
    5
}
