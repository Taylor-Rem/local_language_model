use anyhow::Result;
use std::fs;
use std::path::Path;

/// Pure Rust worker for reading files - no LLM needed
pub struct FileReader;

impl FileReader {
    pub fn new() -> Self {
        Self
    }

    pub fn read(&self, path: &str) -> Result<String> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    pub fn exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }

    pub fn read_lines(&self, path: &str) -> Result<Vec<String>> {
        let content = fs::read_to_string(path)?;
        Ok(content.lines().map(String::from).collect())
    }
}

impl Default for FileReader {
    fn default() -> Self {
        Self::new()
    }
}
