use std::io::Write;

use colored::Colorize;
use deepseek_api::AsyncIteratorNext;
use deepseek_api::{Client, Delta, Function, Model};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize)]
struct AddParamters {
    a: i32,
    b: i32,
}

fn add(args: String) -> String {
    let AddParamters { a, b } = serde_json::from_str(&args).unwrap();
    (a + b).to_string()
}

fn input() -> String {
    print!("{}", "> ".green());
    std::io::stdout().flush().unwrap();

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf
}

#[derive(PartialEq)]
enum State {
    UserInput,
    Thinking,
    ToolCallInput,
    ToolCallOutput,
    Answer,
}

impl State {
    // Print empty line when mode changes.
    fn transition_to(&mut self, mode: Self) {
        use State::*;

        if *self != mode {
            match self {
                UserInput => println!(""),
                ToolCallInput => println!("{}", ")".yellow()),
                ToolCallOutput => println!(""),
                _ => println!("\n"),
            };
            *self = mode
        }
    }
}

#[tokio::main]
async fn main() {
    let api_key = std::env::var("DEEPSEEK_API_KEY").unwrap();
    let mut client = Client::new(Model::DeepSeekReasoner, &api_key);
    client.tools = [Function {
        name: "add".to_string(),
        description: "Adds two integers.".to_string(),
        parameters: schema_for!(AddParamters),
        call: add,
    }]
    .to_vec();

    // Example Input:
    //
    // > Use the provided functions to calculate 114 + 514 and 1919 + (-810).
    //
    // > Alright. Then how about 1 + 1?

    let mut mode = State::UserInput;
    loop {
        mode.transition_to(State::UserInput);
        let prompt = input();
        let mut stream = client.streaming_chat(&prompt).await;

        while let Some(delta) = stream.next().await {
            use Delta::*;
            match delta {
                Assistant {
                    content,
                    reasoning_content,
                    ..
                } => {
                    if let Some(reasoning_content) = reasoning_content {
                        mode.transition_to(State::Thinking);
                        print!("{}", reasoning_content.black());
                    } else if let Some(content) = content {
                        mode.transition_to(State::Answer);
                        print!("{content}");
                    }
                }
                ToolCallInput { tool_calls } => {
                    assert_eq!(tool_calls.len(), 1);
                    let tool_call = &tool_calls[0];
                    if let Some(id) = &tool_call.id {
                        match mode {
                            State::ToolCallInput => println!("{}", ")".yellow()),
                            _ => mode.transition_to(State::ToolCallInput),
                        }
                        print!(
                            "{}{} = {}{}{}",
                            "@".blue(),
                            id.blue(),
                            tool_call.function.name.as_ref().unwrap().yellow(),
                            "(".yellow(),
                            tool_call.function.arguments
                        );
                    } else {
                        print!("{}", tool_call.function.arguments);
                    }
                }
                ToolCallOutput {
                    tool_call_id,
                    content,
                } => {
                    mode.transition_to(State::ToolCallOutput);
                    println!("{}{} = {content}", "@".blue(), tool_call_id.blue());
                }
            }
            std::io::stdout().flush().unwrap();
        }
    }
}
