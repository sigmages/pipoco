use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use anyhow::Result;
use convert_case::{Case, Casing};
use frankenstein::{Api, SendMessageParams, SendPhotoParams, TelegramApi};
use pyo3::{types::PyModule, Python};

pub struct DalleCommand {
    messages: Vec<SendPhotoParams>,
    error_messages: Vec<SendMessageParams>,
    text: String,
    search_term: String,
}

impl DalleCommand {
    pub fn new(text: String) -> Self {
        Self {
            messages: vec![],
            error_messages: vec![],
            text,
            search_term: "".to_string(),
        }
    }
}

fn py_dalle_img_generation(search_term: &str) -> Result<String> {
    let code = std::include_str!("../openaiutil.py");
    Python::with_gil(|py| {
        let activators = PyModule::from_code(py, code, "", "")?;
        let result = activators
            .call_method1("get_image_link", (search_term,))?
            .extract::<String>()?;
        Ok(result)
    })
}

async fn save_img_from_response(search_term: String, url: String) -> PathBuf {
    let resp = reqwest::get(url).await.expect("request failed");
    // saving file
    let search_term = search_term
        .to_case(Case::Snake)
        .get(..10)
        .unwrap_or(&search_term)
        .to_string();
    let filepath = format!("./outputs/{search_term}.png");
    let input = resp.bytes().await.unwrap();
    fs::write(&filepath, input).unwrap();

    // return the file
    let file = std::path::PathBuf::from(filepath);

    file
}

impl DalleCommand {
    pub async fn build(&mut self, chat_id: i64, message_id: i32) -> Result<&Self> {
        let args: Vec<&str> = self.text.split(" ").collect();
        let search_term = args[1..].join(" ");
        self.search_term = search_term.trim().to_owned();
        println!("{}", self.search_term);
        let result = py_dalle_img_generation(&self.search_term);

        if result.is_err() {
            self.error_messages.push(
                SendMessageParams::builder()
                    .chat_id(chat_id)
                    .text(result.err().unwrap().to_string())
                    .reply_to_message_id(message_id)
                    .build(),
            );
            return Ok(self)
        }

        let photo = save_img_from_response(search_term, result?).await;

        self.messages.push(
            SendPhotoParams::builder()
                .chat_id(chat_id)
                .photo(photo)
                .reply_to_message_id(message_id)
                .build(),
        );
        Ok(self)
    }

    pub fn send(&self, api: Api) {
        if !self.error_messages.is_empty() {
            for message in &self.error_messages {
                if let Err(err) = api.send_message(message) {
                    println!("Failed to send message: {:?}", err);
                }
            }
            return
        }
        for message in &self.messages {
            if let Err(err) = api.send_photo(message) {
                println!("Failed to send message: {:?}", err);
            }
        }
    }
}
