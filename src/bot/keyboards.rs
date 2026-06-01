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

pub fn wallet_menu_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("📥 Receive", "receive"),
            InlineKeyboardButton::callback("📤 Send", "send"),
        ],
        vec![InlineKeyboardButton::callback("💰 Balance", "balance")],
        vec![InlineKeyboardButton::callback("⬅️ Back", "back_wallet")],
    ])
}

pub fn cancel_send_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "❌ Cancel",
        "cancel_send",
    )]])
}

pub fn cancel_confirm_send_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback("❌ Cancel", "cancel_send"),
        InlineKeyboardButton::callback("✅ Confirm", "confirm_send"),
    ]])
}

pub fn create_wallet_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "👛 Create Wallet",
        "new_wallet",
    )]])
}
