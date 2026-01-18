use crate::workers::{Archivist, Message};
use crate::workers::traits::{Resettable, Storage};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

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

// =============================================================================
// TRAIT: Display for Conversation
// =============================================================================
// Display lets us print a Conversation nicely. This shows the title and
// message count, which is useful for debugging or showing conversation history.
//
// Note: We already have #[derive(Debug)] which gives us {:?} formatting.
// Display ({}) is for human-readable output.
impl fmt::Display for Conversation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== {} ===", self.title)?;
        for msg in &self.messages {
            // This uses Message's Display impl we created earlier!
            writeln!(f, "{}", msg)?;
        }
        Ok(())
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

    pub fn add_message(&mut self, message: Message) -> Result<()> {
        if let Some(ref mut conv) = self.conversation {
            conv.add_message(message);
        }
        self.save()
    }

}

// =============================================================================
// TRAIT IMPLEMENTATION: Resettable for StateManager
// =============================================================================
// By implementing Resettable, StateManager can be reset through a standard
// interface. This is useful when you have multiple components that need
// resetting and you want to handle them uniformly.
//
// Example of using the trait:
//   fn reset_all(items: &mut [&mut dyn Resettable]) {
//       for item in items {
//           item.reset();
//       }
//   }
impl Resettable for StateManager {
    fn reset(&mut self) {
        self.conversation = None;
    }
}
