use crate::arc::wallet::init::WalletConfig;
use crate::arc::wallet::wallet::create_wallet;
use crate::db::fun::wallet::add_user;

pub mod balance;
mod config;
mod encrypt;
pub mod init;
pub mod send;
pub mod setwallet;
pub mod wallet;

pub async fn arc_create_wallet(
    user_id: &u64,
    db: sqlx::Pool<sqlx::Postgres>,
    wallet_config: WalletConfig,
) -> anyhow::Result<String> {
    let wallet_info = create_wallet(wallet_config).await?;
    let address = wallet_info.address.clone();
    add_user(*user_id as i64, wallet_info, &db).await?;

    Ok(address)
}
