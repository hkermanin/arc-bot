# Telegram Todo Bot

A simple Telegram Todo Bot built with Rust using Teloxide and SQLx.

This project was created mainly for learning:

- Teloxide
- SQLx
- PostgreSQL
- Telegram bot architecture in Rust
- State management with dialogues
- Modular Rust project structure
- Async Rust development

The project is also intended to serve as a reusable template for future Telegram bots.

---

# Features

- Add todos
- List todos
- Inline keyboard menu
- Dialogue/state management
- PostgreSQL database integration
- Modular project structure
- Environment variable configuration with dotenvy

---

# Technologies

- Rust
- Teloxide
- SQLx
- PostgreSQL
- Tokio
- dotenvy

---

# Setup

## Clone the repository

```bash
git clone https://github.com/hkermanin/telegram-todo-bot.git
cd telegram-todo-bot
```

---

## Create `.env`

```env
TELOXIDE_TOKEN=your_bot_token
DATABASE_URL=postgresql://postgres:password@host:5432/database
RUST_LOG=trace
```

---

## Run the project

```bash
cargo run
```

---

# Database

The project uses PostgreSQL with SQLx.

The database table is automatically created on startup if it does not exist.

---

# Goals of This Project

This project was designed to:

- Learn Telegram bot development in Rust
- Learn SQLx and PostgreSQL
- Practice async Rust architecture
- Learn state management with Teloxide dialogues
- Build a reusable template for future Telegram bots

---

# Future Improvements

- Delete todos
- Edit todos
- Better error handling
- Persistent dialogue storage
- SQLx migrations
- Docker support
- Deployment automation

---

# Deployment

The project can be deployed on platforms such as:

- Railway
- Fly.io
- DigitalOcean
- VPS servers

---

# License

MIT
