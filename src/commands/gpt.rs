use anyhow::Result;
use frankenstein::{Api, TelegramApi, SendMessageParams};
use pyo3::{Python, types::PyModule};


pub struct GptCommand {
    messages: Vec<SendMessageParams>,
    text: String,
    search_term: String,
}

impl GptCommand {
    pub fn new(text: String) -> Self {
        Self {
            messages: vec![],
            text,
            search_term: "".to_string(),
        }
    }
}

fn py_gpt_text_completion(search_term: &str) -> Result<String> {
    let code = std::include_str!("../openaiutil.py");
    Python::with_gil(|py| {
        let activators = PyModule::from_code(py, code, "", "")?;
        let result = activators
            .call_method1(
                "get_text_suggestion",
                (
                    search_term,
                ),
            )?
            .extract::<String>()?;
        Ok(result)
    })
}

impl GptCommand {
    pub async fn build(&mut self, chat_id: i64, message_id: i32) -> Result<&Self> {
        let args: Vec<&str> = self.text.split(" ").collect();
        let search_term = args[1..].join(" ");
        self.search_term = search_term.trim().to_owned();
        let text = py_gpt_text_completion(&self.search_term)?;
        println!("{}", text);
        

        self.messages.push(
            SendMessageParams::builder()
                .chat_id(chat_id)
                .text(text)
                .reply_to_message_id(message_id)
                .build(),
        );
        Ok(self)
    }

    pub fn send(&self, api: Api) {
        for message in &self.messages {
            if let Err(err) = api.send_message(message) {
                println!("Failed to send message: {:?}", err);
            }
        }
    }
}
