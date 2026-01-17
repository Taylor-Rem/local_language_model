use std::io;
use std::io::{BufRead, Write};
use anyhow::{Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Config {
    model: String,
    ollama_url: String,
    username: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Conversation {
    title: Option<String>,
    path: String,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    message: Message,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let config: Config = envy::from_env()?;
    let client = reqwest::Client::new();

    let mut system_message = include_str!("prompts/system.txt").to_string();

    if let Some(name) = &config.username {
        system_message.push_str(&format!("\n\nThis user's username is {}. Their home directory is /home/{}. Always use this exact path - do not misspell it.", name, name));
    }

    let system_message = Message {
        role: "system".to_string(),
        content: system_message.to_string(),
    };

    let mut conversation = Conversation {
        title: None,
        path: "../message_history".to_string(),
        messages: vec![system_message]
    };

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("File Browser Assistant (using {})", config.model.clone());
    println!("Type 'quit' to exit\n");

    loop {
        print!("You: ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }
        if input == "quit" || input == "exit" {
            println!("Goodbye!");
            break;
        }
        conversation.messages.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        print!("Assistant: ");
        let request = ChatRequest {
            model: config.model.clone(),
            messages: conversation.messages.clone(),
            stream: false,
        };
        let response: ChatResponse = client
            .post(config.ollama_url.clone())
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        println!("{}", response.message.content);

        conversation.messages.push(response.message);
    }

    Ok(())
}
