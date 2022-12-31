use frankenstein::Api;

pub mod sticker;
pub mod acende;
pub mod game;

pub trait PipocoCommand {
    fn build(&mut self, chat_id: i64) -> &Self;
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
            "/acende" => Self::Acende,
            "/sticker" => Self::Sticker,
            "/game" => Self::Game,
            _ => Self::Unknown,
        }
    }
}
