use anyhow::Result;
use frankenstein::{Api, ParseMode, SendMessageParams, TelegramApi};
use serde::{Deserialize, Serialize};
use pyo3::{types::PyModule, Python};

#[derive(Serialize, Deserialize)]
struct Comment {
    title: String,
    author: String,
    content: String,
    datediff: String,
    country: String,
    score: String,
}

pub struct XComment {
    messages: Vec<SendMessageParams>,
    text: String,
    search_term: String,
}

impl XComment {
    pub fn new(text: String) -> Self {
        Self {
            messages: vec![],
            text,
            search_term: "".to_string(),
        }
    }
}

fn py_xcomment(search_term: &str) -> Result<String> {
    let code = std::include_str!("../randxv.py");
    Python::with_gil(|py| {
        let activators = PyModule::from_code(py, code, "", "")?;
        let result = activators
            .call_method1(
                "choose_random_porn_comment_as_json",
                (
                    search_term,
                ),
            )?
            .extract::<String>()?;
        Ok(result)
    })
}

impl XComment {
    pub async fn build(&mut self, chat_id: i64, message_id: i32) -> Result<&Self> {
        let args: Vec<&str> = self.text.split(" ").collect();
        let search_term = args[1..].join(" ");
        self.search_term = search_term.trim().to_owned();
        let response = py_xcomment(&self.search_term);
        let message = match response {
            Ok(text) => {
                let comment: Comment = serde_json::from_str(&text)?;
                format!(
                    "<b>{}</b>\n<b>{}</b>\n{}\n<i>{}</i>\n{}",
                    comment.title,
                    comment.author,
                    comment.content,
                    comment.country,
                    comment.datediff
                )
            }
            Err(_) => {
                format!("Sem resultados para: <b>{}</b> :(", self.search_term)
            },
        };
        self.messages.push(
            SendMessageParams::builder()
                .chat_id(chat_id)
                .parse_mode(ParseMode::Html)
                .text(message).reply_to_message_id(message_id)
                .build(),
        );
        Ok(self)
    }

    pub async fn send(&self, api: &Api) {
        for message in &self.messages {
            if let Err(err) = api.send_message(message) {
                println!("Failed to send message: {:?}", err);
            }
        }
    }
}
