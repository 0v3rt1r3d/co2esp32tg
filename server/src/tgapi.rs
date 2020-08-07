use serde::{Deserialize, Serialize};

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

pub fn send_image(
    token: &str,
    chat_id: &str,
    // image_base64: &str // TODO: send from buffer
    filename: &str
) {
    let client = reqwest::blocking::Client::new();
    let form = reqwest::blocking::multipart::Form::new()
        .text("chat_id", chat_id.to_string())
        // .part("photo", reqwest::blocking::multipart::Part::bytes());
        .file("photo", filename)
        .unwrap();

    let request = client.post(&format!("https://api.telegram.org/bot{}/sendPhoto", token))
        .multipart(form)
        .build()
        .unwrap();
    println!("{:?}", request);

    let result = client.execute(request).unwrap();
    println!("Result: {}, {}", result.status(), result.text().unwrap());
}
