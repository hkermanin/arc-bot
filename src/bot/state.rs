use teloxide::types::MessageId;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Main,
    Wallet,
    Send(SendState),
}

#[derive(Clone)]
pub enum SendState {
    WaitingRecipient {
        prompt_message_id: MessageId,
    },
    WaitingAmount {
        recipient: String,
        prompt_message_id: MessageId,
    },
    WaitingConfirmation {
        recipient: String,
        amount: String,
        prompt_message_id: MessageId,
    },
}
