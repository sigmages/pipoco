use frankenstein::{Api, TelegramApi, SendMessageParams};

use super::PipocoCommand;

pub struct GameCommand {
    messages: Vec<SendMessageParams>,
    text: String,
}

impl GameCommand {
    pub fn new(text: String) -> Self {
        Self { messages: vec![], text }
    }
}

impl PipocoCommand for GameCommand {
    fn build(&mut self, chat_id: i64) -> &Self {
        self.messages.push(
            SendMessageParams::builder()
                .chat_id(chat_id)
                .text(self.text.clone())
                .build(),
        );
        self
    }

    fn send(&self, api: &Api) {
        for message in &self.messages {
            if let Err(err) = api.send_message(message) {
                println!("Failed to send message: {:?}", err);
            }
        }
    }
}