use crate::arc::wallet::init::WalletConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::arc::wallet::encrypt::ciphertext;

#[derive(Deserialize)]
pub struct CircleErrorResponse {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct TransferResponse {
    pub data: TransferData,
}

#[derive(Debug, Deserialize)]
pub struct TransferData {
    pub id: String,
    pub state: String,
}

#[derive(Serialize)]
pub struct TransferRequest {
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: String,

    #[serde(rename = "entitySecretCiphertext")]
    pub entity_secret_ciphertext: String,

    #[serde(rename = "walletId")]
    pub wallet_id: String,

    pub blockchain: String,

    #[serde(rename = "destinationAddress")]
    pub destination_address: String,

    pub amounts: Vec<String>,

    #[serde(rename = "feeLevel")]
    pub fee_level: String,
}

pub async fn send_transaction(
    user_id: i64,
    recipient: String,
    amount: String,
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> anyhow::Result<String> {
    let wallet_id = sqlx::query_scalar("SELECT wallet_id FROM users WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&db)
        .await?;

    let entity_secret_ciphertext =
        ciphertext(&wallet_config.public_key, &wallet_config.entity_secret)?;

    let body = TransferRequest {
        idempotency_key: Uuid::new_v4().to_string(),
        entity_secret_ciphertext,
        wallet_id,
        blockchain: "ARC-TESTNET".to_string(),
        destination_address: recipient,
        amounts: vec![amount],
        fee_level: "MEDIUM".to_string(),
    };

    let client = Client::new();

    let response = client
        .post("https://api.circle.com/v1/w3s/developer/transactions/transfer")
        .bearer_auth(&wallet_config.api_key)
        .json(&body)
        .send()
        .await?;

    let is_success = response.status().is_success();
    let response_text = response.text().await?;

    if is_success {
        let tx: TransferResponse = serde_json::from_str(&response_text)?;

        Ok(format!("✅ Transaction submitted successfully.",))
    } else {
        let err: CircleErrorResponse = serde_json::from_str(&response_text)?;

        Ok(format!("❌ Transfer Failed\n\n{}", err.message))
    }
}
