use serde::{Deserialize, Serialize};

pub struct Drafter;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Drafter {
    pub fn new() -> Self {
        Self
    }

    pub fn create_message(&self, role: String, content: String) -> Message {
        Message { role, content }
    }
}