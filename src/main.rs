mod agents;
mod workers;

use std::io;
use std::io::{BufRead, Write};
use anyhow::Result;
use serde::Deserialize;

use agents::{AgentManager, ChatAgent, Conversationalist, Orchestrator, OrchestratorDecision, Labeler};
use workers::{Archivist, Drafter, StateManager};

#[derive(Debug, Deserialize)]
struct Config {
    labeler: String,
    conversationalist: String,
    orchestrator: String,
    ollama_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let config: Config = envy::from_env()?;

    let labeler = Labeler::new(
        config.labeler.clone(),
        config.ollama_url.clone(),
        include_str!("prompts/labeler.txt").to_string(),
    );

    // Set up agent manager
    let mut agent_manager = AgentManager::new();

    // Register conversationalist agent
    let conversationalist = Conversationalist::new(
        config.conversationalist.clone(),
        config.ollama_url.clone(),
        include_str!("prompts/conversationalist.txt").to_string(),
        "conversationalist".to_string(),
        "For general conversation, brainstorming, advice, and discussion".to_string(),
    );
    agent_manager.register(Box::new(conversationalist));

    // Set up orchestrator with registered agents
    let orchestrator = Orchestrator::new(
        config.orchestrator.clone(),
        config.ollama_url.clone(),
        agent_manager.list(),
        5, // max iterations
    );

    let drafter = Drafter::new();
    let archivist = Archivist::new(None);
    let mut state = StateManager::new(archivist);

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("Local LLM Assistant");
    println!("Orchestrator: {} | Conversationalist: {}", config.orchestrator, config.conversationalist);
    println!("Type 'quit' to exit\n");

    loop {
        print!("You: ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() { continue; }
        if input == "quit" || input == "exit" {
            state.save()?;
            println!("Goodbye!");
            break;
        }

        let user_message = drafter.create_message("user".to_string(), input.to_string());

        // First iteration: create title and conversation
        if !state.has_conversation() {
            let title = labeler.label_conversation(&user_message).await?;
            // Use the default agent's system message to start the conversation
            let default_agent = agent_manager.default_agent()
                .expect("At least one agent must be registered");
            let system_message = default_agent.system_message();
            state.start_conversation(title, system_message);
            state.save()?;
        }

        // Add user message to conversation
        state.add_message(user_message)?;

        // Get messages for routing and processing
        let messages = &state.conversation_mut().unwrap().messages;

        // Route the request
        let agent_choice = orchestrator.route(messages).await?;

        // Process with the chosen agent
        let agent = agent_manager.get(&agent_choice)
            .or_else(|| {
                println!("[Orchestrator chose: {}, defaulting to conversationalist]", agent_choice);
                agent_manager.get("conversationalist")
            })
            .expect("Conversationalist agent must be registered");

        // Agent processes messages and returns response (doesn't modify state)
        let messages = &state.conversation_mut().unwrap().messages;
        let mut response = agent.chat(messages).await?;

        // Evaluation loop with max iterations
        let mut iterations = 0;
        loop {
            if iterations >= orchestrator.max_iterations() {
                println!("[Max iterations reached]");
                break;
            }

            let decision = orchestrator.evaluate(input, &response.content).await?;

            match decision {
                OrchestratorDecision::Complete => break,
                OrchestratorDecision::NeedsMoreWork(reason) => {
                    println!("[Orchestrator: needs more work - {}]", reason);

                    // Add current response and follow-up to conversation
                    state.add_message(response)?;
                    let follow_up = drafter.create_message(
                        "user".to_string(),
                        format!("Please elaborate on your previous response. The issue: {}", reason),
                    );
                    state.add_message(follow_up)?;

                    // Get updated messages and ask agent again
                    let messages = &state.conversation_mut().unwrap().messages;
                    response = agent.chat(messages).await?;
                    iterations += 1;
                }
                OrchestratorDecision::RouteToAgent(_) => break, // Shouldn't happen in evaluate
            }
        }

        // Add final response to conversation and save
        println!("Assistant: {}", response.content);
        state.add_message(response)?;
    }

    Ok(())
}
