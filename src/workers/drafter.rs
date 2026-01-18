use serde::{Deserialize, Serialize};
use std::fmt;

pub struct Drafter;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

// =============================================================================
// TRAIT: Default
// =============================================================================
// The `Default` trait is from Rust's standard library. It provides a way to
// create a "default" value for a type. This is useful when:
// - You need a placeholder value
// - You want to use struct update syntax: SomeStruct { field: value, ..Default::default() }
// - Working with Option<T>::unwrap_or_default()
//
// For Drafter, there's no configuration needed, so the default is just an empty struct.
impl Default for Drafter {
    fn default() -> Self {
        Self
    }
}

impl Drafter {
    // `new()` can now just call Default. This is a common pattern in Rust.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_message(&self, role: String, content: String) -> Message {
        Message { role, content }
    }
}

// =============================================================================
// TRAIT: Display
// =============================================================================
// The `Display` trait controls how a type is formatted when you use {} in
// println!, format!, etc. It's meant for user-facing output.
//
// Compare with Debug (which uses {:?}) - Debug is for programmers, Display is for users.
//
// Example usage after implementing:
//   let msg = Message { role: "user".into(), content: "Hello".into() };
//   println!("{}", msg);  // prints: [user] Hello
impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.role, self.content)
    }
}

// =============================================================================
// TRAIT: From<T>
// =============================================================================
// The `From` trait enables type conversions. When you implement From<T> for YourType,
// you automatically get Into<YourType> for T as well (Rust does this for you).
//
// This is useful for:
// - Making APIs more ergonomic (accept multiple input types)
// - The ? operator uses From for error conversion
//
// Here we implement From<(&str, &str)> so you can create a Message from a tuple:
//   let msg: Message = ("user", "Hello").into();
//   let msg = Message::from(("user", "Hello"));
impl From<(&str, &str)> for Message {
    fn from((role, content): (&str, &str)) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}

// Also implement From for owned Strings - this shows you can implement
// the same trait multiple times for different input types
impl From<(String, String)> for Message {
    fn from((role, content): (String, String)) -> Self {
        Self { role, content }
    }
}