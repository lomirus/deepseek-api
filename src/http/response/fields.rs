use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "insufficient_system_resource")]
    InsufficientSystemResource,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    completion_tokens: u64,
    prompt_tokens: u64,
    prompt_cache_hit_tokens: u64,
    prompt_cache_miss_tokens: u64,
    total_tokens: u64,
    completion_tokens_details: Option<CompletionTokensDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionTokensDetails {
    reasoning_tokens: u64,
}
