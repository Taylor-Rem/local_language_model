// =============================================================================
// CUSTOM TRAITS
// =============================================================================
// This file contains custom traits that define shared behavior across our
// workers. Traits are Rust's way of defining shared behavior - similar to
// interfaces in other languages, but more powerful.
//
// WHY USE TRAITS?
// 1. Abstraction: Define what something CAN DO without specifying HOW
// 2. Polymorphism: Write code that works with any type implementing a trait
// 3. Extensibility: Add new implementations without changing existing code
// 4. Testing: Easy to create mock implementations for tests
// =============================================================================

use anyhow::Result;
use crate::workers::Conversation;

// =============================================================================
// TRAIT: Storage
// =============================================================================
// This trait defines what it means to be a "storage backend" for conversations.
// By using a trait instead of a concrete type, we can:
// - Swap file storage for database storage without changing other code
// - Create a mock storage for testing
// - Support multiple storage backends simultaneously
//
// The current Archivist implements file-based storage, but you could create:
// - DatabaseStorage (stores in SQLite, Postgres, etc.)
// - MemoryStorage (for testing, doesn't persist)
// - CloudStorage (stores in S3, etc.)
//
// Example of using this trait as a parameter:
//   fn save_work<S: Storage>(storage: &S, conv: &Conversation) -> Result<()> {
//       storage.save(conv)
//   }
// This function works with ANY type that implements Storage!
pub trait Storage {
    /// Save a conversation to storage
    fn save(&self, conversation: &Conversation) -> Result<()>;

    /// Load a conversation by filename/identifier
    fn load(&self, identifier: &str) -> Result<Conversation>;

    /// List all stored conversations
    fn list(&self) -> Result<Vec<String>>;
}

// =============================================================================
// TRAIT: Named
// =============================================================================
// A simple trait for anything that has a name. This demonstrates:
// - Traits can be very simple (just one method)
// - You can use traits to group things conceptually
//
// Both agents and workers could implement this, allowing code like:
//   fn print_name<T: Named>(item: &T) {
//       println!("Name: {}", item.name());
//   }
pub trait Named {
    fn name(&self) -> &str;
}

// =============================================================================
// TRAIT: Described
// =============================================================================
// For components that have a description. Combined with Named, you get
// a pattern for self-documenting components.
//
// Note: You can require other traits! The `: Named` means any type
// implementing Described must ALSO implement Named.
pub trait Described: Named {
    fn description(&self) -> &str;
}

// =============================================================================
// TRAIT: Resettable
// =============================================================================
// For components that can be reset to their initial state.
// This shows traits with &mut self (mutable methods).
pub trait Resettable {
    /// Reset to initial state
    fn reset(&mut self);
}
