use anyhow::Result;
use super::agent::Agent;
use crate::workers::Message;

pub struct Labeler {
    agent: Agent,
}

impl Labeler {
    pub fn new(model: String, ollama_url: String, system_prompt: String) -> Self {
        let agent = Agent::new(model, ollama_url, system_prompt);
        Self { agent }
    }

    pub async fn label_conversation(&self, message: &Message) -> Result<String> {
        let messages = vec![self.agent.system_message(), message.clone()];
        let response = self.agent.chat(&messages).await?;
        Ok(response.content)
    }
}
