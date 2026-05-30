use anyhow::Result;
use reqwest::Client;

use crate::arc::wallet::config::arc_config;
use crate::arc::wallet::setwallet::create_set_wallet;
use crate::db::fun::walletconfig::{add_wallet_id, find_wallet_id};

#[derive(Clone)]
pub struct WalletConfig {
    pub api_key: String,
    pub entity_secret: String,
    pub public_key: String,
    pub wallet_name: String,
    pub wallet_id: String,
}

pub async fn init_arc_wallet(db: &sqlx::Pool<sqlx::Postgres>) -> Result<WalletConfig> {
    let client = Client::new();

    let config = arc_config(&client).await?;

    if let Some(wallet) = find_wallet_id(db).await? {
        return Ok(WalletConfig {
            api_key: config.api_key,
            entity_secret: config.entity_secret,
            public_key: config.public_key,
            wallet_name: wallet.wallet_name,
            wallet_id: wallet.wallet_id,
        });
    } else {
        let wallet_set = create_set_wallet("main-arcbot-walletset", &client, &config).await?;
        add_wallet_id(wallet_set, db).await?;
        if let Some(wallet) = find_wallet_id(db).await? {
            return Ok(WalletConfig {
                api_key: config.api_key,
                entity_secret: config.entity_secret,
                public_key: config.public_key,
                wallet_name: wallet.wallet_name,
                wallet_id: wallet.wallet_id,
            });
        } else {
            panic!("Wallet set can not load from DB");
        }
    }
}
