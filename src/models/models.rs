use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatGPTRequest {
    pub model: String,
    pub messages: Vec<ChatGPTMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatGPTMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatGPTResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub message: ChatGPTMessage,
}