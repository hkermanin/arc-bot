use reqwest::Client;
use serde::Deserialize;

use crate::arc::wallet::init::WalletConfig;

#[derive(Debug, Deserialize)]
pub struct BalanceResponse {
    pub data: BalanceData,
}

#[derive(Debug, Deserialize)]
pub struct BalanceData {
    #[serde(rename = "tokenBalances")]
    pub token_balances: Vec<TokenBalance>,
}

#[derive(Debug, Deserialize)]
pub struct TokenBalance {
    pub token: Token,
    pub amount: String,
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub symbol: String,

    #[serde(rename = "isNative")]
    pub is_native: bool,
}

pub async fn show_balance(
    user_id: i64,
    db: &sqlx::Pool<sqlx::Postgres>,
    wallet_config: &WalletConfig,
) -> anyhow::Result<String> {
    let wallet_id: String = sqlx::query_scalar(
        "SELECT wallet_id FROM users WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;

    let client = Client::new();

    let response = client
        .get(format!(
            "https://api.circle.com/v1/w3s/wallets/{}/balances",
            wallet_id
        ))
        .bearer_auth(&wallet_config.api_key)
        .send()
        .await?
        .error_for_status()?;

    let body = response.text().await?;

    let balance: BalanceResponse = serde_json::from_str(&body)?;

    if balance.data.token_balances.is_empty() {
        return Ok(
            "💰 Wallet Balance\n\nNo assets found in this wallet."
                .to_string(),
        );
    }

    let mut message = String::from("💰 Wallet Balance\n\n");

    for token in balance.data.token_balances {
        let token_type = if token.token.is_native {
            "Native"
        } else {
            "ERC20"
        };

        message.push_str(&format!(
            "• {} ({}): {}\n",
            token.token.symbol,
            token_type,
            token.amount
        ));
    }

    Ok(message)
}