use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn menu_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback("Add", "add"),
        InlineKeyboardButton::callback("List", "list"),
    ]])
}

pub fn create_wallet_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "Create new Wallet",
        "new_wallet",
    )]])
}
