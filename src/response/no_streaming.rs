use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Model,
    response::fields::{FinishReason, Message, Usage},
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
    finish_reason: FinishReason,
    pub message: Message,
    logprobs: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Object {
    #[serde(rename = "chat.completion")]
    ChatCompletion,
}
