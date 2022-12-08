use frankenstein::GetUpdatesParams;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use frankenstein::{Api, UpdateContent};
use rand::Rng;

static TOKEN: &str = "";

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
                        if message.text.unwrap_or("".to_string()) == "/acende" {
                            let messages = build_messages(message.chat.id);
                            send_messages(&messages, &api);
                        } else {

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
