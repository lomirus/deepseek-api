use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Model, Role, ToolCallType, response::fields::{FinishReason, Usage}
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    id: String,
    pub choices: Vec<Choice>,
    created: u64,
    model: Model,
    system_fingerprint: String,
    object: Object,
    usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    index: u64,
    pub finish_reason: FinishReason,
    pub message: Message,
    logprobs: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub content: String,
    pub reasoning_content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    role: Role,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Object {
    #[serde(rename = "chat.completion")]
    ChatCompletion,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    index: u32,
    pub id: String,
    pub r#type: ToolCallType,
    pub function: Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arguments: String,
}
