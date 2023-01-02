pub mod algorithm;
mod commands;

use commands::game::GameSession;
use frankenstein::AllowedUpdate;
use frankenstein::BotCommand;
use frankenstein::GetUpdatesParams;
use frankenstein::SendMessageParams;
use frankenstein::SetMyCommandsParams;
use frankenstein::TelegramApi;
use frankenstein::{Api, UpdateContent};

use crate::commands::acende::AcendeCommand;
use crate::commands::game::GameCommand;
use crate::commands::sticker::StickerCommand;
use crate::commands::CommandType;
use crate::commands::PipocoCommand;

static TOKEN: &str = "5922619577:AAHGRGbHTcYonxmQEQbR7MMeEVZa57p_0rY";


fn main() {
    let api = Api::new(TOKEN);

    let update_params_builder = GetUpdatesParams::builder().allowed_updates(vec![
        AllowedUpdate::CallbackQuery,
        AllowedUpdate::Message,
        AllowedUpdate::InlineQuery,
        AllowedUpdate::ChosenInlineResult,
    ]);
    let mut update_params = update_params_builder.clone().build();

    // configure commands
    let params = SetMyCommandsParams::builder()
        .commands(vec![
            BotCommand {
                command: "acende".to_string(),
                description: "acende o rojão".to_string(),
            },
            BotCommand {
                command: "sticker".to_string(),
                description: "sticker aleatorio".to_string(),
            },
            BotCommand {
                command: "game".to_string(),
                description: "desafia alguem para o jogo da velha".to_string(),
            },
        ])
        .build();

    let _ = api.set_my_commands(&params);
    let mut game_session = GameSession::new();
    loop {
        let result = api.get_updates(&update_params);
        println!("{:?}", result);
        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content.clone() {
                        let command: CommandType =
                            message.text.clone().unwrap_or("".to_string()).into();
                        match command {
                            CommandType::Acende => AcendeCommand::new()
                                .build(message.chat.id)
                                .unwrap()
                                .send(&api),
                            CommandType::Sticker => StickerCommand::new()
                                .build(message.chat.id)
                                .unwrap()
                                .send(&api),
                            CommandType::Game => {
                                // always clear the session when a new game are created
                                game_session = GameSession::new();
                                let game_command = GameCommand::new(
                                    message.text.unwrap_or_default(),
                                    message.from.unwrap().username.unwrap_or_default(),
                                );
                                if let Ok(mut command) = game_command {
                                    command
                                        .build(message.chat.id, &mut game_session)
                                        .unwrap()
                                        .send(&api)
                                } else {
                                    let text = game_command.err().unwrap().to_string();
                                    let _ = api.send_message(
                                        &SendMessageParams::builder()
                                            .chat_id(message.chat.id)
                                            .text(text)
                                            .build(),
                                    );
                                }
                            }
                            // nothing to do...
                            CommandType::Unknown => (),
                        }
                    }
                    if let UpdateContent::CallbackQuery(message) = update.content.clone() {
                        let username = message.from.username.unwrap_or_default();
                        if !game_session.players.contains_key(&username) {
                            // prevent non users doing plays, just ignore
                            update_params = update_params_builder
                                .clone()
                                .offset(update.update_id + 1)
                                .build();
                            continue;
                        }
                        let mut game_command =
                            GameCommand::new("".to_string(), "".to_string()).unwrap();
                        let result = game_command.reply_player_movement(
                            message.message.clone().unwrap().chat.id,
                            message.data.unwrap(),
                            username.clone(),
                            &mut game_session,
                        );
                        if let Ok(x) = result {
                            x.send(&api);
                        } else {
                            let _ = api.send_message(
                                &SendMessageParams::builder()
                                    .chat_id(message.message.clone().unwrap().chat.id)
                                    .text(result.err().unwrap().to_string())
                                    .build(),
                            );
                        }
                        // check for a winner
                        if let Some(_) = game_session.board.find_winner() {
                            let _ = api.send_message(
                                &SendMessageParams::builder()
                                    .chat_id(message.message.clone().unwrap().chat.id)
                                    .text(format!("{} foi o vencedor!", username.clone()))
                                    .build(),
                            );
                            // clear game session
                            game_session = GameSession::new();
                        }
                        // println!("callback: {:?}", message.data.unwrap());
                    }
                    update_params = update_params_builder
                        .clone()
                        .offset(update.update_id + 1)
                        .build();
                }
            }
            Err(error) => {
                println!("Failed to get updates: {:?}", error);
            }
        }
    }
}
