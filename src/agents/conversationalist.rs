use anyhow::Result;
use async_trait::async_trait;
use super::agent::Agent;
use super::chat_agent::ChatAgent;
use crate::workers::Message;

pub struct Conversationalist {
    agent: Agent,
    name: String,
    description: String,
}

impl Conversationalist {
    pub fn new(
        model: String,
        ollama_url: String,
        system_prompt: String,
        name: String,
        description: String,
    ) -> Self {
        let agent = Agent::new(model, ollama_url, system_prompt);
        Self { agent, name, description }
    }

    pub fn model(&self) -> &str {
        self.agent.model()
    }
}

#[async_trait]
impl ChatAgent for Conversationalist {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn system_message(&self) -> Message {
        self.agent.system_message()
    }

    /// Process messages and return response.
    /// Note: This agent does NOT modify conversation state.
    /// The caller (main/orchestrator) is responsible for adding messages.
    async fn chat(&self, messages: &[Message]) -> Result<Message> {
        self.agent.chat(messages).await
    }
}
