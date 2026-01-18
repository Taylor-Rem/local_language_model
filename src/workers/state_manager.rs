use crate::workers::{Archivist, Message};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Represents a conversation with messages
#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub title: String,
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new(title: String, system_message: Message) -> Self {
        Self {
            title,
            messages: vec![system_message],
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}
pub struct StateManager {
    conversation: Option<Conversation>,
    archivist: Archivist,
}

impl StateManager {
    pub fn new(archivist: Archivist) -> Self {
        Self {
            conversation: None,
            archivist,
        }
    }

    pub fn has_conversation(&self) -> bool {
        self.conversation.is_some()
    }

    pub fn start_conversation(&mut self, title: String, system_message: Message) {
        self.conversation = Some(Conversation::new(title, system_message));
    }

    pub fn conversation_mut(&mut self) -> Option<&mut Conversation> {
        self.conversation.as_mut()
    }

    pub fn save(&self) -> Result<()> {
        if let Some(ref conv) = self.conversation {
            self.archivist.save(conv)?;
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        self.conversation = None;
    }
}
