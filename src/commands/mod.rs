use frankenstein::Api;

pub mod sticker;
pub mod acende;
pub mod game;
use anyhow::Result;

pub trait PipocoCommand {
    fn build(&mut self, chat_id: i64) -> Result<&Self>;
    fn send(&self, api: &Api);
}


pub enum CommandType {
    Acende,
    Sticker,
    Game,
    Unknown
}

impl From<String> for CommandType {
    fn from(value: String) -> Self {
        let cmd: Vec<&str> = value.split(" ").collect();
        let cmd = cmd.into_iter().nth(0).unwrap_or_default();
        match cmd {
            "/acende" | "/acende@rojaum_bot" => Self::Acende,
            "/sticker" | "/sticker@rojaum_bot" => Self::Sticker,
            "/game" | "/game@rojaum_bot" => Self::Game,
            _ => Self::Unknown,
        }
    }
}
