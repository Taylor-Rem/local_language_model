use anyhow::Result;
use std::fs;
use std::path::Path;
use crate::workers::Conversation;
use crate::workers::traits::{Storage, Named};

/// Pure Rust worker for saving/loading chat history - no LLM needed
pub struct Archivist {
    storage_path: String,
}

impl Archivist {
    pub fn new(storage_path: Option<String>) -> Self {
        let path = storage_path.unwrap_or_else(|| "./message_history".to_string());

        // Ensure storage directory exists
        let _ = fs::create_dir_all(&path);

        Self { storage_path: path }
    }
}

// =============================================================================
// TRAIT IMPLEMENTATION: Storage for Archivist
// =============================================================================
// This is where traits become powerful. By implementing the Storage trait,
// Archivist promises to provide save/load/list functionality.
//
// Now ANY code that needs storage can accept `impl Storage` or `&dyn Storage`
// and work with Archivist OR any future storage implementation.
//
// Example - this function works with any storage:
//   fn backup<S: Storage>(from: &S, to: &S) -> Result<()> {
//       for id in from.list()? {
//           let conv = from.load(&id)?;
//           to.save(&conv)?;
//       }
//       Ok(())
//   }
impl Storage for Archivist {
    fn save(&self, conversation: &Conversation) -> Result<()> {
        let filename = format!("{}.json", conversation.title.replace(" ", "_"));
        let path = Path::new(&self.storage_path).join(&filename);
        let json = serde_json::to_string_pretty(conversation)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn load(&self, filename: &str) -> Result<Conversation> {
        let path = Path::new(&self.storage_path).join(filename);
        let content = fs::read_to_string(path)?;
        let record: Conversation = serde_json::from_str(&content)?;
        Ok(record)
    }

    fn list(&self) -> Result<Vec<String>> {
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

// =============================================================================
// TRAIT IMPLEMENTATION: Named for Archivist
// =============================================================================
// Simple trait implementation - just returns the worker's name.
// This could be useful for logging, debugging, or UI purposes.
impl Named for Archivist {
    fn name(&self) -> &str {
        "archivist"
    }
}
