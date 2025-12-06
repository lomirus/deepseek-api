use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Model, Role, ToolCallType, response::fields::FinishReason};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chunk {
    pub id: String,
    pub choices: Vec<Choice>,
    pub created: u64,
    pub model: Model,
    pub system_fingerprint: String,
    pub object: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    pub index: u64,
    pub finish_reason: Option<FinishReason>,
    pub delta: Delta,
    pub logprobs: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged, deny_unknown_fields)]
pub enum Delta {
    Assistant {
        content: Option<String>,
        reasoning_content: Option<String>,
        role: Option<Role>,
    },
    ToolCall {
        tool_calls: Vec<ToolCall>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub r#type: Option<ToolCallType>,
    pub id: Option<String>,
    pub index: usize,
    pub function: Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    pub name: Option<String>,
    pub arguments: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Object {
    #[serde(rename = "chat.completion.chunk")]
    ChatCompletionChunk,
}
