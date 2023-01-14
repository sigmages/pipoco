use frankenstein::{Api, SendMessageParams, TelegramApi};

use anyhow::Result;

use crate::commands::dollar;

pub struct DollarToBRLCommand {
    messages: Vec<SendMessageParams>,
}

impl DollarToBRLCommand {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }
}

impl DollarToBRLCommand {
    pub async fn build(
        &mut self,
        chat_id: i64,
        message_id: i32,
        text_input: Option<String>,
    ) -> Result<&Self> {
        let message_text = text_input.unwrap_or_default();
        let parts: Vec<&str> = message_text.split(" ").collect();
        let dollar_amount = parts.get(1).unwrap_or(&"1").parse::<f64>().unwrap_or(1.0);
        let to_currency = parts.get(2).unwrap_or(&"BRL");

        let response: serde_json::Value =
            reqwest::get("https://api.exchangerate-api.com/v4/latest/USD")
                .await?
                .json()
                .await?;

        let text;

        if !response["rates"]
            .as_object()
            .unwrap()
            .contains_key(to_currency.to_uppercase().as_str())
        {
            text = format!("Código '{}' de moeda inválido", to_currency);
        } else {
            let exchange_rate = response["rates"][to_currency.to_uppercase()]
                .as_f64()
                .unwrap();

            let amount_in_currency = dollar_amount * exchange_rate;
            let dollar_word = match (dollar_amount.eq(&1.0), to_currency.to_uppercase()) {
                (true, _) => "dólar",
                (false, _) => "dólares",
            };

            text = if to_currency == &"BRL" {
                format!(
                    "{} {} equivale a R$ {:.2} reais",
                    dollar_amount, dollar_word, amount_in_currency
                )
            } else {
                format!(
                    "{} {} equivale a {:.2} {}",
                    dollar_amount,
                    dollar_word,
                    amount_in_currency,
                    to_currency.to_uppercase()
                )
            };
        }

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
