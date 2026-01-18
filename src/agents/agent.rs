use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::workers::Message;

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    message: Message,
}

pub struct Agent {
    model: String,
    ollama_url: String,
    client: reqwest::Client,
    system_prompt: String,
}

impl Agent {
    pub fn new(model: String, ollama_url: String, system_prompt: String) -> Self {
        Self {
            model,
            ollama_url,
            client: reqwest::Client::new(),
            system_prompt,
        }
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn system_message(&self) -> Message {
        Message {
            role: "system".to_string(),
            content: self.system_prompt.clone(),
        }
    }

    pub async fn chat(&self, messages: &[Message]) -> Result<Message> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages: messages.to_vec(),
            stream: false,
        };

        let response: ChatResponse = self
            .client
            .post(&self.ollama_url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.message)
    }
}
