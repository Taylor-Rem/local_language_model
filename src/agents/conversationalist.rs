use anyhow::Result;
use super::agent::Agent;
use crate::workers::{Message, Conversation};

pub struct Conversationalist {
    agent: Agent,
}

impl Conversationalist {
    pub fn new(model: String, ollama_url: String, system_prompt: String) -> Self {
        let agent = Agent::new(model, ollama_url, system_prompt);
        Self { agent }
    }

    pub fn model(&self) -> &str {
        self.agent.model()
    }

    pub fn system_message(&self) -> Message {
        self.agent.system_message()
    }

    pub async fn chat(&self, conversation: &mut Conversation, user_input: &str) -> Result<String> {
        // Add user message to conversation
        let user_message = Message {
            role: "user".to_string(),
            content: user_input.to_string(),
        };
        conversation.add_message(user_message);

        // Call agent.chat with conversation messages
        let response = self.agent.chat(&conversation.messages).await?;
        let content = response.content.clone();

        // Add response to conversation
        conversation.add_message(response);

        Ok(content)
    }
}
