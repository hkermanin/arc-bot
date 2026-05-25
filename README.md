# Telegram Todo Bot

A simple Telegram Todo Bot built with Rust using Teloxide and SQLx.

This project was created mainly for learning:

- Teloxide
- SQLx
- SQLite
- Telegram bot architecture in Rust
- State management with dialogues
- Modular Rust project structure

The project is also intended to serve as a reusable template for future Telegram bots.

---

# Features

- Add todos
- List todos
- Inline keyboard menu
- Dialogue/state management
- SQLite database integration
- Modular project structure

---

# Technologies

- Rust
- Teloxide
- SQLx
- SQLite
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
DATABASE_URL=sqlite://database.db
RUST_LOG=trace
```

---

## Run the project

```bash
cargo run
```

---

# Database

The project uses SQLite with SQLx.

The database file is automatically created on startup if it does not exist.

---

# Goals of This Project

This project was designed to:

- Learn Telegram bot development in Rust
- Learn SQLx and SQLite
- Practice async Rust architecture
- Build a reusable template for future Telegram bots

---

# Future Improvements

- Delete todos
- Edit todos
- Better error handling
- Persistent dialogue storage
- Migrations
- Docker support

---

# License

MIT
