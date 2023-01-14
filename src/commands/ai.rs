use std::{path::PathBuf, fs, sync::Arc};

use anyhow::Result;
use frankenstein::{Api, SendPhotoParams, TelegramApi};
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use base64::{Engine as _, engine::general_purpose};

use crate::constants::DEFAULT_NEGATIVE_PROMPT;

#[derive(Serialize, Deserialize)]
struct Payload {
    prompt: String,
    steps: i32,
    width: i32,
    height: i32,
    negative_prompt: String,
}

pub struct AiCommand {
    messages: Vec<SendPhotoParams>,
    text: String,
    search_term: String,
}

impl AiCommand {
    pub fn new(text: String) -> Self {
        Self {
            messages: vec![],
            text,
            search_term: "".to_string(),
        }
    }
}

fn save_img_from_response(search_term: String, value: Value) -> PathBuf {
    // saving file
    let search_term = search_term.to_case(Case::Snake);
    let search_term = search_term.get(..10).unwrap_or(&search_term).to_string();
    let filepath = format!("./outputs/{search_term}.png");
    let input = value["images"][0].as_str().unwrap();
    let decode = general_purpose::STANDARD.decode(input).unwrap();
    fs::write(&filepath, decode).unwrap();
    
    // return the file
    let file = std::path::PathBuf::from(filepath);

    file
}

impl AiCommand {
    pub async fn build(&mut self, chat_id: i64, message_id: i32) -> Result<&Self> {
        let args: Vec<&str> = self.text.split(" ").collect();
        let search_term = args[1..].join(" ");
        self.search_term = search_term.trim().to_owned();

        let payload = Payload {
            prompt: self.search_term.clone(),
            steps: 20,
            width: 512,
            height: 720,
            negative_prompt: DEFAULT_NEGATIVE_PROMPT.to_string(),
        };
        let client = reqwest::Client::new();
        let response = client
            .post("http://127.0.0.1:7860/sdapi/v1/txt2img")
            .body(serde_json::to_string(&payload).unwrap())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let photo = save_img_from_response(self.search_term.clone(), response);

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
        for message in &self.messages {
            if let Err(err) = api.send_photo(message) {
                println!("Failed to send message: {:?}", err);
            }
        }
    }
}


#[cfg(test)]
pub mod test {
    use std::{fs::{File, self}, io::BufReader};
    use base64::{Engine as _, engine::general_purpose};

    use serde_json::Value;

    #[test]
    fn test_decode_file() {
        let path = "./src/fixtures/test_ai_response.json";
        let filepath = "./outputs/testfile.jpeg";
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        
        let value: Value = serde_json::from_reader(reader).unwrap();
        let input = value["images"][0].as_str().unwrap();
        let decode = general_purpose::STANDARD.decode(input).unwrap();
        fs::write(filepath, decode).unwrap();
    }
}

