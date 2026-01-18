mod agents;
mod workers;

use std::io;
use std::io::{BufRead, Write};
use anyhow::Result;
use serde::Deserialize;

use agents::{Conversationalist, Orchestrator, AgentInfo, OrchestratorDecision, Labeler};
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

    // Set up conversationalist agent
    let conversationalist = Conversationalist::new(
        config.conversationalist.clone(),
        config.ollama_url.clone(),
        include_str!("prompts/conversationalist.txt").to_string(),
    );

    // Set up orchestrator with agent descriptions
    let orchestrator = Orchestrator::new(
        config.orchestrator.clone(),
        config.ollama_url.clone(),
        vec![
            AgentInfo {
                name: "conversationalist".to_string(),
                description: "For general conversation, brainstorming, advice, and discussion".to_string(),
            },
            // Add more agents here as you create them
        ],
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
            let system_message = conversationalist.system_message();
            state.start_conversation(title, system_message);
            state.save()?;
        }

        let conv = state.conversation_mut().unwrap();

        // Route the request
        let agent_choice = orchestrator.route(&user_message).await?;

        // Process with the chosen agent
        let mut response = match agent_choice.as_str() {
            "conversationalist" => conversationalist.chat(conv, input).await?,
            _ => {
                println!("[Orchestrator chose: {}, defaulting to conversationalist]", agent_choice);
                conversationalist.chat(conv, input).await?
            }
        };

        // Evaluation loop with max iterations
        let mut iterations = 0;
        loop {
            if iterations >= orchestrator.max_iterations() {
                println!("[Max iterations reached]");
                break;
            }

            let decision = orchestrator.evaluate(input, &response).await?;

            match decision {
                OrchestratorDecision::Complete => break,
                OrchestratorDecision::NeedsMoreWork(reason) => {
                    println!("[Orchestrator: needs more work - {}]", reason);
                    // For now, just ask conversationalist to elaborate
                    let follow_up = format!("Please elaborate on your previous response. The issue: {}", reason);
                    response = conversationalist.chat(conv, &follow_up).await?;
                    iterations += 1;
                }
                OrchestratorDecision::RouteToAgent(_) => break, // Shouldn't happen in evaluate
            }
        }

        println!("Assistant: {}", response);

        // Save periodically
        state.save()?;
    }

    Ok(())
}
