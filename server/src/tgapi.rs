use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chat {
    pub id: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub text: String,
    pub chat: Chat
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Update {
    pub message: Message,
}

pub fn send_message(
    token: &str,
    chat_id: &str,
    text: &str,
) {
    let client = reqwest::blocking::Client::new();
    let result = client.post(&format!("https://api.telegram.org/bot{}/sendMessage", token))
        .body(format!(
            "{{ \
                \"parse_mode\":\"MarkdownV2\", \
                \"chat_id\":{}, \
                \"text\":\"{}\" \
            }}",
            chat_id,
            text,
        ))
        .header("Content-Type", "application/json")
        .send()
        .unwrap();

    println!("Result: {}, {}", result.status(), result.text().unwrap());
}
