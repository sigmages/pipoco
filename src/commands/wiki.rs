use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use frankenstein::{Api, ParseMode, SendMessageParams, TelegramApi};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Comment {
    title: String,
    author: String,
    content: String,
    datediff: String,
    country: String,
    score: String,
}

pub struct WikiCommand {
    messages: Vec<SendMessageParams>,
    text: String,
    search_term: String,
}

impl WikiCommand {
    pub fn new(text: String) -> Self {
        Self {
            messages: vec![],
            text,
            search_term: "".to_string(),
        }
    }
}

async fn pt_wiki_fallback(terms: &Vec<String>) -> Result<String> {
    let mut introduction: Result<String> = Err(anyhow!("Termo não encontrado"));
    for term in terms {
        let response = reqwest::get(&format!(
            "https://pt.wikipedia.org/api/rest_v1/page/summary/{}",
            term
        ))
        .await?
        .json::<serde_json::Value>()
        .await?;

        let extract = response["extract"].as_str();
        if extract.is_some() {
            introduction = Ok(response["extract"].as_str().unwrap().to_string());
            break;
        }
    }
    introduction
}

async fn en_wiki_fallback(terms: &Vec<String>) -> Result<String> {
    let mut introduction: Result<String> = Err(anyhow!("Termo não encontrado"));
    for term in terms {
        let response = reqwest::get(&format!(
            "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
            term
        ))
        .await?
        .json::<serde_json::Value>()
        .await?;

        let extract = response["extract"].as_str();
        if extract.is_some() {
            introduction = Ok(response["extract"].as_str().unwrap().to_string());
            break;
        }
    }
    introduction
}

impl WikiCommand {
    pub async fn build(&mut self, chat_id: i64, message_id: i32) -> Result<&Self> {
        let args: Vec<&str> = self.text.split(" ").collect();
        let search_term = args[1..].join(" ");
        self.search_term = search_term;

        let terms = vec![
            self.search_term.clone(),
            self.search_term
                .clone()
                .to_case(Case::Train)
                .replace("-", "_"), // Snake_Pascal_Case
        ];

        // try pt wikipedia
        let result = pt_wiki_fallback(&terms)
            .await
            .or(en_wiki_fallback(&terms).await);
        let introduction = result.unwrap_or("Termo não encontrado".to_string());

        self.messages.push(
            SendMessageParams::builder()
                .chat_id(chat_id)
                .parse_mode(ParseMode::Html)
                .text(introduction)
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
