use anyhow::Result;
use async_trait::async_trait;
use crate::workers::Message;

/// Trait that all chat-capable agents must implement.
/// This enables the AgentManager to store and route to agents polymorphically.
///
/// DESIGN: Agents are stateless processors. They receive messages and return
/// a response. They do NOT manage conversation state - that's the caller's job
/// (main.rs or orchestrator). This separation keeps agents simple and testable.
#[async_trait]
pub trait ChatAgent: Send + Sync {
    /// Unique identifier for this agent (used for routing)
    fn name(&self) -> &str;

    /// Human-readable description of what this agent does
    fn description(&self) -> &str;

    /// Returns the system message that initializes this agent's persona
    fn system_message(&self) -> Message;

    /// Process messages and return a response.
    /// Takes a slice of messages (read-only) and returns the assistant's response.
    /// The caller is responsible for adding messages to the conversation.
    async fn chat(&self, messages: &[Message]) -> Result<Message>;
}
