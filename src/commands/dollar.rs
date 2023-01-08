use frankenstein::{Api, SendMessageParams, TelegramApi};

use anyhow::Result;

pub struct DollarToBRLCommand {
    messages: Vec<SendMessageParams>,
}

impl DollarToBRLCommand {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }
}

impl DollarToBRLCommand {
    pub async fn build(&mut self, chat_id: i64, message_id: i32) -> Result<&Self> {
        let response: serde_json::Value = reqwest::get("https://api.exchangerate-api.com/v4/latest/USD").await?
            .json().await?;

        let exchange_rate = response["rates"]["BRL"].as_f64().unwrap();
        let text = format!("Hoje, 1 Dólar equivale a {exchange_rate} Reais");
        self.messages.push(
            SendMessageParams::builder()
                .chat_id(chat_id)
                .text(text)
                .reply_to_message_id(message_id)
                .build(),
        );

        Ok(self)
    }

    pub fn send(&self, api: &Api) {
        for message in &self.messages {
            if let Err(err) = api.send_message(message) {
                println!("Failed to send message: {:?}", err);
            }
        }
    }
}
