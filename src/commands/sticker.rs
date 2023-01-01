use std::fs;

use frankenstein::{Api, SendStickerParams, TelegramApi};
use rand::Rng;
use anyhow::Result;

use super::PipocoCommand;

pub struct StickerCommand {
    messages: Vec<SendStickerParams>,
}

impl StickerCommand {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }
}

impl PipocoCommand for StickerCommand {
    fn build(&mut self, chat_id: i64) -> Result<&Self> {
        let base_path = "./bichostrostes/png";
        let dir = fs::read_dir(&base_path).unwrap();
        let size = dir.count();
        let mut dir = fs::read_dir(&base_path).unwrap();
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..size);
        let filename = dir
            .nth(n)
            .expect("should return a file")
            .unwrap()
            .file_name();
        let filename = filename.to_str().unwrap();
        let sticker = format!("{}/{}", base_path, filename);
        let file = std::path::PathBuf::from(sticker.clone());
        let message = SendStickerParams::builder()
            .chat_id(chat_id)
            .sticker(file)
            .build();
        self.messages = vec![message];

        Ok(self)
    }

    fn send(&self, api: &Api) {
        for message in &self.messages {
            if let Err(err) = api.send_sticker(message) {
                println!("Failed to send message: {:?}", err);
            }
        }
    }
}
