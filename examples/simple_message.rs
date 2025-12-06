use deepseek_api::{Client, Message, Model};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("DEEPSEEK_API_KEY").unwrap();
    let mut client = Client::new(Model::DeepSeekChat, &api_key);
    let answers = client.chat("Hello!").await;

    // It must be 1 since there is no tool calls.
    assert_eq!(answers.len(), 1);

    let answer = match &answers[0] {
        Message::Assistant(assistant) => &assistant.content,
        _ => unreachable!(),
    };

    println!("{answer}");
}
