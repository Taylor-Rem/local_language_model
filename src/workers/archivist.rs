use anyhow::Result;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::workers::Message;
use crate::workers::Conversation;

/// Pure Rust worker for saving/loading chat history - no LLM needed
pub struct Archivist {
    storage_path: String,
}

impl Archivist {
    pub fn new(storage_path: Option<String>) -> Self {
        let path = storage_path.unwrap_or_else(|| "../message_history".to_string());

        // Ensure storage directory exists
        let _ = fs::create_dir_all(&path);

        Self { storage_path: path }
    }

    pub fn save(&self, conversation: &Conversation) -> Result<()> {
        let filename = format!("{}.json", conversation.title.replace(" ", "_"));
        let path = Path::new(&self.storage_path).join(&filename);
        let json = serde_json::to_string_pretty(conversation)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load(&self, filename: &str) -> Result<Conversation> {
        let path = Path::new(&self.storage_path).join(filename);
        let content = fs::read_to_string(path)?;
        let record: Conversation = serde_json::from_str(&content)?;
        Ok(record)
    }

    pub fn list_conversations(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
        for entry in fs::read_dir(&self.storage_path)? {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".json") {
                    files.push(name.to_string());
                }
            }
        }
        Ok(files)
    }
}
