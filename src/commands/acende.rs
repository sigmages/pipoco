use frankenstein::{Api, SendMessageParams, TelegramApi};
use rand::Rng;

use super::PipocoCommand;
use anyhow::Result;

pub struct AcendeCommand {
    messages: Vec<SendMessageParams>,
}

impl AcendeCommand {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }
}

impl PipocoCommand for AcendeCommand {
    fn build(&mut self, chat_id: i64) -> Result<&Self> {
        let mut rng = rand::thread_rng();
        let rows = rng.gen_range(1..5);
        let mut messages: Vec<SendMessageParams> = vec![];
        for _ in 0..rows {
            let pows_per_row = rng.gen_range(1..5);
            let row_text = String::from(" pra").repeat(pows_per_row);
            messages.push(
                SendMessageParams::builder()
                    .chat_id(chat_id)
                    .text(row_text)
                    .build(),
            );
        }
        let last_pow_text: String;

        // check if last pow "gorou"
        if rng.gen_bool(0.3) {
            last_pow_text = "...".to_string();
        }
        // check if last pow "bota o fuzil pra cantar"
        else if rng.gen_bool(0.3) {
            last_pow_text = "BOTA O FUZIL PRA CANTAR PA PUM!!!".to_string();
        } else {
            last_pow_text = "POOOOWWWW!!!".to_string();
        }
        messages.push(
            SendMessageParams::builder()
                .chat_id(chat_id)
                .text(last_pow_text)
                .build(),
        );
        self.messages = messages;

        Ok(self)
    }

    fn send(&self, api: &Api) {
        for message in &self.messages {
            if let Err(err) = api.send_message(message) {
                println!("Failed to send message: {:?}", err);
            }
        }
    }
}
