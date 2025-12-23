use std::io::Write;

use colored::Colorize;
use deepseek_api::AsyncIteratorNext;
use deepseek_api::{Client, Delta, Model};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("DEEPSEEK_API_KEY").unwrap();
    let mut client = Client::new(Model::DeepSeekReasoner, &api_key);
    let mut stream = client.streaming_chat("Hello!").await;

    let mut is_thinking = true;
    while let Some(delta) = stream.next().await {
        match delta {
            Delta::Thinking {
                reasoning_content, ..
            } => {
                print!("{}", reasoning_content.bright_black());
                std::io::stdout().flush().unwrap();
            }
            Delta::Content { content, .. } => {
                if is_thinking {
                    is_thinking = false;
                    println!("\n");
                }
                print!("{content}");
                std::io::stdout().flush().unwrap();
            }
            _ => unreachable!(),
        }
    }
}
