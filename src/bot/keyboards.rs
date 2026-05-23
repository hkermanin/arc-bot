use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn menu_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback("Add", "add"),
        InlineKeyboardButton::callback("List", "list"),
    ]])
}
