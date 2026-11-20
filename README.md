# ArcCircleBot

ArcCircleBot is a Telegram bot built in Rust for creating wallets, checking
balances, and sending USDC on Arc Testnet. It uses Circle Developer-Controlled
Wallets and PostgreSQL.

Telegram handle: [@arccirclebot](https://t.me/arccirclebot)

## What you can do

- Create an Arc Testnet wallet from the Telegram menu.
- View your wallet balance and share your receive address as a QR code.
- Review the recipient and amount before confirming a USDC transfer.

## Run locally

You’ll need Rust and Cargo (the repo pins Rust 1.89.0), a PostgreSQL database,
a Telegram bot token from BotFather, and a Circle API key with a registered
entity secret.

```sh
git clone https://github.com/hkermanin/arc-bot.git
cd arc-bot
```

Create a `.env` file in the repository root:

```dotenv
TELOXIDE_TOKEN=your_telegram_bot_token
DATABASE_URL=postgresql://username:password@localhost:5432/arc_bot
CIRCLE_API_KEY=your_circle_api_key
ENTITY_SECRET=your_registered_entity_secret
RUST_LOG=info
```

Keep real credentials in your local `.env` file. The repository ignores this
file. Create the PostgreSQL database first and give the configured user
permission to create tables.

```sh
cargo run --locked
```

On startup, the bot creates any missing tables. It reuses the Circle wallet
set stored in PostgreSQL, or creates one if the database has no saved set.
Once the bot is running, send `/start` to the Telegram bot tied to your token.

## Code layout

- `src/bot/` handles Telegram messages, menus, and transfer confirmation.
- `src/arc/wallet/` handles Circle API calls for wallet creation, balances,
  and transfers.
- `src/db/` stores the wallet set and maps Telegram users to their wallets.

This project is intended for Arc Testnet.
