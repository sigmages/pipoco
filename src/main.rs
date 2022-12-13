use std::borrow::Borrow;
use std::fs;

use frankenstein::GetUpdatesParams;
use frankenstein::SendMessageParams;
use frankenstein::SendStickerParams;
use frankenstein::TelegramApi;
use frankenstein::{Api, UpdateContent};
use rand::Rng;

static TOKEN: &str = "5922619577:AAHGRGbHTcYonxmQEQbR7MMeEVZa57p_0rY";
fn build_sticker(chat_id: i64) -> Vec<SendStickerParams> {
    let base_path = "./bichostrostes/png";
    let dir = fs::read_dir(&base_path).unwrap();
    let size = dir.count();
    let mut dir = fs::read_dir(&base_path).unwrap();
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..size);
    let filename = dir.nth(n).expect("should return a file").unwrap().file_name();
    let filename = filename.to_str().unwrap();
    let sticker = format!("{}/{}", base_path, filename);
    let file = std::path::PathBuf::from(sticker.clone());
    let message = SendStickerParams::builder().chat_id(chat_id).sticker(file).build();
    vec![message]
}
fn build_messages(chat_id: i64) -> Vec<SendMessageParams> {
    let mut rng = rand::thread_rng();
    let rows = rng.gen_range(1..5);
    let mut messages: Vec<SendMessageParams> = vec![];
    for _ in 0..rows {
        let pows_per_row = rng.gen_range(1..5);
        let row_text = String::from(" pra").repeat(pows_per_row);
        messages.push(
            SendMessageParams::builder()
                .chat_id(chat_id)
                .text(row_text)
                .build(),
        );
    }
    let last_pow_text: String;

    // check if last pow "gorou"
    if rng.gen_bool(0.3) {
        last_pow_text = "...".to_string();
    }
    // check if last pow "bota o fuzil pra cantar"
    else if rng.gen_bool(0.3) {
        last_pow_text = "BOTA O FUZIL PRA CANTAR PA PUM!!!".to_string();
    }
    else {
        last_pow_text = "POOOOWWWW!!!".to_string();
    }
    messages.push(
        SendMessageParams::builder()
            .chat_id(chat_id)
            .text(last_pow_text)
            .build(),
    );
    messages
}

fn send_messages(messages: &Vec<SendMessageParams>, api: &Api) {
    for message in messages {
        if let Err(err) = api.send_message(message) {
            println!("Failed to send message: {:?}", err);
        }
    }
}
fn send_stickers(messages: &Vec<SendStickerParams>, api: &Api) {
    for message in messages {
        if let Err(err) = api.send_sticker(message) {
            println!("Failed to send message: {:?}", err);
        }
    }
}
fn main() {
    let api = Api::new(TOKEN);

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    loop {
        let result = api.get_updates(&update_params);

        println!("result: {:?}", result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        if message.text.clone().unwrap_or("".to_string()) == "/acende" {
                            let messages = build_messages(message.chat.id);
                            send_messages(&messages, &api);
                        } else if message.text.clone().unwrap_or("".to_string()) == "/sticker" {
                            let messages = build_sticker(message.chat.id);
                            send_stickers(&messages, &api);
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
