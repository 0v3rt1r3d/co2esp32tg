use serde::{Deserialize, Serialize};

fn default_timeout() -> std::time::Duration {
    return std::time::Duration::new(10,0);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chat {
    pub id: u64
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
) -> std::result::Result<(), reqwest::Error>{
    let client = reqwest::blocking::Client::new();
    client.post(&format!("https://api.telegram.org/bot{}/sendMessage", token))
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
        .timeout(default_timeout())
        .send()?;
    return Ok(());
}

pub fn send_image(
    token: &str,
    chat_id: &str,
    filename: &str
) -> std::result::Result<(), Box<dyn std::error::Error>>{
    let client = reqwest::blocking::Client::new();
    let form = reqwest::blocking::multipart::Form::new()
        .text("chat_id", chat_id.to_string())
        .file("photo", filename)?;

    let request = client.post(&format!("https://api.telegram.org/bot{}/sendPhoto", token))
        .multipart(form)
        .timeout(default_timeout())
        .build()?;
        

    client.execute(request)?;
    return Ok(());
}

pub trait TgEscapable {
    fn escape_tg(&self) -> String;
}

impl TgEscapable for String {
    fn escape_tg(&self) -> String {
        self
            .replace("-", "\\\\-")
            .replace(".", "\\\\.")
            .replace("+", "\\\\+")
            .replace("(", "\\\\(")
            .replace(")", "\\\\)")
    }
}
