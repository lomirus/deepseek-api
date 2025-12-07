pub mod message;

use schemars::Schema;
use serde::{Deserialize, Serialize};

use crate::Model;

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub messages: Vec<message::Message>,
    pub model: Model,
    pub thinking: Option<Thinking>,
    pub stream: bool,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    pub frequency_penalty: Option<f32>,

    /// The maximum number of tokens that can be generated in the chat completion.
    ///
    /// The total length of input tokens and generated tokens is limited by the model's context length.
    ///
    /// For the value range and default value, please refer to the [documentation](https://api-docs.deepseek.com/quick_start/pricing).
    pub max_tokens: Option<u32>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    pub presence_penalty: Option<f32>,

    /// An object specifying the format that the model must output. Setting to { "type": "json_object" } enables JSON Output, which guarantees the message the model generates is valid JSON.
    ///
    /// Important: When using JSON Output, you must also instruct the model to produce JSON yourself via a system or user message. Without this, the model may generate an unending stream of whitespace until the generation reaches the token limit, resulting in a long-running and seemingly "stuck" request. Also note that the message content may be partially cut off if finish_reason="length", which indicates the generation exceeded max_tokens or the conversation exceeded the max context length.
    pub response_format: ResponseFormat,

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or `temperature`` but not both.
    pub top_p: Option<f32>,

    /// A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported.
    pub tools: Vec<Tool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Thinking {
    pub r#type: ThinkingType,
}

impl Thinking {
    pub const fn enabled() -> Self {
        Thinking {
            r#type: ThinkingType::Enabled,
        }
    }

    pub const fn disabled() -> Self {
        Thinking {
            r#type: ThinkingType::Disabled,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ThinkingType {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "function")]
#[serde(rename_all = "snake_case")]
pub enum Tool {
    Function {
        name: String,
        description: String,
        parameters: Schema,
    },
}

impl From<crate::Function> for Tool {
    fn from(value: crate::Function) -> Self {
        Self::Function {
            name: value.name,
            description: value.description,
            parameters: value.parameters,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseFormat {
    r#type: ResponseFormatType,
}

impl From<ResponseFormatType> for ResponseFormat {
    fn from(value: ResponseFormatType) -> Self {
        ResponseFormat { r#type: value }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    Text,
    JsonObject,
}
