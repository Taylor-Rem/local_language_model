use anyhow::Result;
use super::agent::Agent;
use super::agent_manager::AgentInfo;
use crate::workers::Message;

pub enum OrchestratorDecision {
    RouteToAgent(String),
    Complete,
    NeedsMoreWork(String),
}

pub struct Orchestrator {
    agent: Agent,
    available_agents: Vec<AgentInfo>,
    max_iterations: usize,
}

impl Orchestrator {
    pub fn new(
        model: String,
        ollama_url: String,
        agents: Vec<AgentInfo>,
        max_iterations: usize,
    ) -> Self {
        let agent_list = agents
            .iter()
            .map(|a| format!("- {}: {}", a.name, a.description))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"You are an orchestrator that routes user requests and evaluates responses.

Available agents:
{}

You have two jobs:

1. ROUTING: When given a user request (no response yet), decide which agent should handle it.
   Reply with just the agent name, nothing else.

2. EVALUATING: When given a request AND a response, decide if the request is complete.
   Reply with either:
   - "complete" if the response fully addresses the request
   - "incomplete: <reason>" if more work is needed

Be concise. One word for routing, one line for evaluation."#,
            agent_list
        );

        Self {
            agent: Agent::new(model, ollama_url, prompt),
            available_agents: agents,
            max_iterations,
        }
    }

    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    /// Decide which agent should handle the request
    pub async fn route(&self, messages: &Vec<Message>) -> Result<String> {
        println!("Deciding what to do...");

        let response = self.agent.chat(&messages).await?;
        let choice = response.content.trim().to_lowercase();

        println!("Agent: {}", choice);

        // Validate it's a known agent
        let valid_names: Vec<_> = self.available_agents.iter().map(|a| a.name.to_lowercase()).collect();
        if valid_names.contains(&choice) {
            Ok(choice)
        } else {
            // Default to first agent (conversationalist)
            Ok(self.available_agents.first()
                .map(|a| a.name.to_lowercase())
                .unwrap_or_else(|| "conversationalist".to_string()))
        }
    }

    /// Evaluate if the response completes the original request
    pub async fn evaluate(&self, original_request: &str, response: &str) -> Result<OrchestratorDecision> {
        let messages = vec![
            self.agent.system_message(),
            Message {
                role: "user".to_string(),
                content: format!(
                    "Original request: {}\n\nResponse: {}\n\nIs this complete?",
                    original_request, response
                ),
            },
        ];
        println!("Validating response...");
        let result = self.agent.chat(&messages).await?;
        let content = result.content.trim().to_lowercase();

        if content.starts_with("complete") {
            println!("Responding to user.");
            Ok(OrchestratorDecision::Complete)
        } else if content.starts_with("incomplete") {
            let reason = content
                .strip_prefix("incomplete:")
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "needs more work".to_string());
            println!("{}", reason);
            Ok(OrchestratorDecision::NeedsMoreWork(reason))
        } else {
            // Assume complete if we can't parse
            Ok(OrchestratorDecision::Complete)
        }
    }
}
