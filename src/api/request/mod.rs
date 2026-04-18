use schemars::Schema;
use serde::{Deserialize, Serialize};

use crate::{Model, api::ToolCallType};

#[derive(Serialize)]
pub struct ChatCompletionRequest {
    pub messages: Vec<Message>,
    pub model: Model,
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
    pub temperature: f32,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or `temperature` but not both.
    pub top_p: f32,

    /// A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported.
    pub tools: Vec<Tool>,
}

#[derive(Serialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Message {
    System {
        name: Option<String>,
        content: String,
    },
    User {
        name: Option<String>,
        content: String,
    },
    Assistant {
        name: Option<String>,
        content: String,
        reasoning_content: Option<String>,
        tool_calls: Option<Vec<ToolCall>>,
    },
    Tool {
        tool_call_id: String,
        content: String,
    },
}

impl From<&crate::message::Message> for Message {
    fn from(value: &crate::message::Message) -> Self {
        match value {
            crate::message::Message::System(s) => Message::System {
                name: s.name.clone(),
                content: s.content.clone(),
            },
            crate::message::Message::User(u) => Message::User {
                name: u.name.clone(),
                content: u.content.clone(),
            },
            crate::message::Message::Assistant(a) => Message::Assistant {
                name: a.name.clone(),
                content: a.content.clone(),
                reasoning_content: a.reasoning_content.clone(),
                tool_calls: a
                    .tool_calls
                    .as_ref()
                    .map(|tcs| tcs.iter().map(ToolCall::from).collect()),
            },
            crate::message::Message::Tool(t) => Message::Tool {
                tool_call_id: t.tool_call_id.clone(),
                content: t.content.clone(),
            },
        }
    }
}

#[derive(Serialize)]
pub struct ToolCall {
    pub r#type: ToolCallType,
    pub id: String,
    pub function: Function,
}

impl From<&crate::message::ToolCall> for ToolCall {
    fn from(value: &crate::message::ToolCall) -> Self {
        Self {
            r#type: ToolCallType::Function,
            id: value.id.clone(),
            function: Function {
                name: value.function.name.clone(),
                arguments: value.function.arguments.clone(),
            },
        }
    }
}

#[derive(Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: String,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "function")]
#[serde(rename_all = "snake_case")]
pub enum Tool {
    Function {
        name: &'static str,
        description: &'static str,
        parameters: &'static Schema,
    },
}

impl From<&crate::Tool> for Tool {
    fn from(value: &crate::Tool) -> Self {
        Self::Function {
            name: value.name,
            description: value.description,
            parameters: (value.parameters)(),
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

impl From<crate::ResponseFormat> for ResponseFormatType {
    fn from(value: crate::ResponseFormat) -> Self {
        match value {
            crate::ResponseFormat::Text => ResponseFormatType::Text,
            crate::ResponseFormat::JsonObject => ResponseFormatType::JsonObject,
        }
    }
}

impl From<crate::ResponseFormat> for ResponseFormat {
    fn from(value: crate::ResponseFormat) -> Self {
        ResponseFormat {
            r#type: value.into(),
        }
    }
}
