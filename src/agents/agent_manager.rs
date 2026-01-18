use std::collections::HashMap;
use super::chat_agent::ChatAgent;

/// Information about a registered agent (for orchestrator routing)
pub struct AgentInfo {
    pub name: String,
    pub description: String,
}

/// Manages all chat-capable agents and provides routing capabilities
pub struct AgentManager {
    agents: HashMap<String, Box<dyn ChatAgent>>,
}

// =============================================================================
// TRAIT: Default for AgentManager
// =============================================================================
// Default creates an empty AgentManager. This is the idiomatic way to provide
// a "constructor" for types with sensible defaults.
//
// Benefits:
// - Works with #[derive(Default)] on structs containing AgentManager
// - Can use AgentManager::default() or Default::default()
// - Enables patterns like Option<AgentManager>::unwrap_or_default()
impl Default for AgentManager {
    fn default() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }
}

impl AgentManager {
    // new() now delegates to Default - this is a common Rust pattern
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an agent with the manager
    pub fn register(&mut self, agent: Box<dyn ChatAgent>) {
        let name = agent.name().to_string();
        self.agents.insert(name, agent);
    }

    /// Get an agent by name
    pub fn get(&self, name: &str) -> Option<&dyn ChatAgent> {
        self.agents.get(name).map(|a| a.as_ref())
    }

    /// Get a mutable reference to an agent by name
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Box<dyn ChatAgent>> {
        self.agents.get_mut(name)
    }

    /// List all registered agents (for orchestrator to use)
    pub fn list(&self) -> Vec<AgentInfo> {
        self.agents
            .values()
            .map(|agent| AgentInfo {
                name: agent.name().to_string(),
                description: agent.description().to_string(),
            })
            .collect()
    }

    /// Get the default agent (first registered, or None)
    pub fn default_agent(&self) -> Option<&dyn ChatAgent> {
        self.agents.values().next().map(|a| a.as_ref())
    }
}
