use std::collections::HashMap;

use frankenstein::{
    Api, InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup, SendMessageParams,
    TelegramApi,
};

use crate::{
    algorithm::{algorithm::TTTBoard, enums::PlayerShape},
};
use anyhow::{bail, Result};

pub struct GameSession {
    pub board: TTTBoard,
    pub players: HashMap<String, PlayerShape>,
    pub current_player: String,
}

impl GameSession {
    pub fn new() -> Self {
        Self {
            board: TTTBoard::new(),
            players: HashMap::new(),
            current_player: "".to_string(),
        }
    }
}

pub struct GameCommand {
    message: Option<SendMessageParams>,
    text: String,
    username: String,
    owner: String,
    adversary: String,
}

impl GameCommand {
    pub fn new(text: String, username: String) -> Result<Self> {

        Ok(Self {
            message: None,
            text,
            username,
            owner: "".to_string(),
            adversary: "".to_string(),
        })
    }
    pub fn reply_player_movement(&mut self, chat_id: i64, callback: String, username: String, session: &mut GameSession) -> Result<&Self> {
        // do the player movement
        let chordinates: Vec<&str> = callback.split(",").collect();
        let (x, y) = (
            chordinates[0].parse::<usize>().unwrap(),
            chordinates[1].parse::<usize>().unwrap(),
        );

        if username != session.current_player {
            bail!("Jogador não pode fazer duas jogadas no mesmo turno");
        }
        let player = session.players.get(&username).unwrap().clone();
        session.board.insert(x, y, &player);

        // alternate player turn
        let current = session
            .players
            .iter()
            .filter(|x| x.0 != &username)
            .last()
            .unwrap();
        session.current_player = current.0.clone();

        // render the message to resend the board
        let inline_keyboard = render_telegram_board(&session).unwrap();
        let reply_markup = InlineKeyboardMarkup::builder()
            .inline_keyboard(inline_keyboard)
            .build();

        let message = SendMessageParams::builder()
            .chat_id(chat_id)
            .text(self.alternate_turn_text(&session))
            .reply_markup(ReplyMarkup::InlineKeyboardMarkup(reply_markup))
            .build();
        self.message = Some(message);

        Ok(self)
    }

    fn wellcome_text(&self) -> String {
        format!(
            "O {} desafiou {} para o Jogo da Velha\n {} = X, {} = O",
            self.owner, self.adversary, self.owner, self.adversary
        )
    }
    fn alternate_turn_text(&self, session: &GameSession) -> String {
        let current_player = &session.current_player;
        format!(
            "O Turno de {}",
            current_player
        )
    }
}

impl GameCommand {
    pub fn build(&mut self, chat_id: i64, session: &mut GameSession) -> Result<&Self> {
        self.owner = self.username.clone();
        let args: Vec<&str> = self.text.split(" ").collect();
        if args.len() != 2 {
            bail!("É necessário desafiar alguém para jogar, ex: /game @adversario");
        }
        self.adversary = args.into_iter().last().unwrap_or_default().to_string();
        // remove the @ mention in username
        self.adversary.remove(0);

        // setup the game session
        session.players.insert(self.owner.clone(), PlayerShape::Cross);
        session.players.insert(self.adversary.clone(), PlayerShape::Circle);
        session.current_player = self.owner.clone();

        let inline_keyboard = render_telegram_board(&session).unwrap();
        let reply_markup = InlineKeyboardMarkup::builder()
            .inline_keyboard(inline_keyboard)
            .build();

        let message = SendMessageParams::builder()
            .chat_id(chat_id)
            .text(self.wellcome_text())
            .reply_markup(ReplyMarkup::InlineKeyboardMarkup(reply_markup))
            .build();
        self.message = Some(message);
        Ok(self)
    }

    pub fn send(&self, api: &Api) {
        let message = self.message.as_ref().expect("should have message to send");
        if let Err(err) = api.send_message(message) {
            println!("Failed to send message: {:?}", err);
        }
    }
}

fn render_telegram_board(session: &GameSession) -> Result<Vec<Vec<InlineKeyboardButton>>> {
    let board = session.board.matrix;

    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    for (i, row) in board.iter().enumerate() {
        let mut row_btn: Vec<InlineKeyboardButton> = Vec::new();

        for (j, col) in row.iter().enumerate() {
            let chordinates = format!("{i},{j}");
            let shape = PlayerShape::from_integer(*col)?;
            let button = InlineKeyboardButton::builder()
                .text(shape.to_string())
                .callback_data(chordinates)
                .build();

            row_btn.push(button);
        }

        keyboard.push(row_btn);
    }
    Ok(keyboard)
}
