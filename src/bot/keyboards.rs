use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn main_menu_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("👛 Wallet", "wallet")],
        vec![
            InlineKeyboardButton::callback("📈 Trade", "trade"),
            InlineKeyboardButton::callback("🤖 AI Analysis", "ai_analysis"),
        ],
    ])
}

pub fn create_wallet_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "👛 Create Wallet",
        "new_wallet",
    )]])
}
