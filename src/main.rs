mod commands;

use frankenstein::BotCommand;
use frankenstein::GetUpdatesParams;
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

    let update_params_builder = GetUpdatesParams::builder();
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
        ])
        .build();

    let _ = api.set_my_commands(&params);

    loop {
        let result = api.get_updates(&update_params);

        println!("result: {:?}", result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content.clone() {
                        let command: CommandType = message.text.clone().unwrap_or("".to_string()).into();
                        match command {
                            CommandType::Acende => {
                                AcendeCommand::new().build(message.chat.id).send(&api)
                            }
                            CommandType::Sticker => {
                                StickerCommand::new().build(message.chat.id).send(&api)
                            }
                            CommandType::Game => {
                                GameCommand::new(message.text.unwrap_or_default()).build(message.chat.id).send(&api)
                                
                            }
                            CommandType::Unknown => println!("nothing to do..."),
                        }
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
