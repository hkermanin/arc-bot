# ArcCircleBot

**Telegram Bot:** `@arccirclebot`

Try the bot directly on Telegram and create your own ARC testnet wallet in seconds.

---

## Overview

ArcCircleBot is a Telegram-based crypto wallet built with Rust and powered by Circle Developer-Controlled Wallets.

The bot allows users to create and manage wallets directly from Telegram without needing to interact with complex blockchain tools or browser extensions.

### Current Features

- Wallet creation on ARC Testnet
- Wallet balance retrieval
- Asset transfers between wallets
- User wallet management
- Secure Circle API integration
- PostgreSQL-backed persistence

---

## Features

### Wallet Management

- Create a new wallet
- Store wallet information securely
- Retrieve wallet details when needed

### Balance Tracking

- View wallet balances
- Display token balances available in the wallet

### Asset Transfers

- Send assets to another wallet address
- Confirmation step before execution
- Transaction status reporting

### Infrastructure

- Rust async backend
- PostgreSQL database
- Circle Developer-Controlled Wallets
- RSA-OAEP + SHA256 encryption for Entity Secret handling
- Modular architecture for future expansion

---

## Technology Stack

### Backend

- Rust
- Tokio
- Reqwest
- Serde
- SQLx
- Anyhow

### Database

- PostgreSQL

### Blockchain Infrastructure

- Circle Developer-Controlled Wallets
- ARC Testnet

---

## Project Architecture

```text
Telegram
    │
    ▼
Bot Handlers
    │
    ▼
Application Layer
    │
    ├── Wallet Service
    ├── Transfer Service
    ├── Balance Service
    │
    ▼
PostgreSQL
    │
    ▼
Circle APIs
    │
    ▼
ARC Testnet
```

---

## Security

The project follows several security practices:

- Developer-controlled wallets managed by Circle
- Encrypted Entity Secret handling
- Idempotent transaction requests
- Database-backed wallet ownership mapping
- Separation of configuration and business logic

---

## Current Status

### Implemented

- Wallet Set initialization
- Wallet creation
- User database integration
- Wallet menu system
- Balance retrieval
- Asset transfers
- Transaction response handling

---

## Development

This project is actively under development and serves as the foundation for a broader ARC ecosystem assistant on Telegram.
