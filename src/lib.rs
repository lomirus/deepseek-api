mod request;
mod response;

use async_stream::stream;
use futures::{StreamExt, stream::BoxStream};
use request::ChatCompletionRequest;
use response::streaming::Chunk;
use schemars::Schema;
use serde::{Deserialize, Serialize};

pub use response::{FinishReason, streaming};

use crate::{
    request::Thinking,
    response::{UserBalance, no_streaming},
};

const BASE_URL: &str = "https://api.deepseek.com";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Model {
    #[serde(rename = "deepseek-chat")]
    DeepSeekChat,
    #[serde(rename = "deepseek-reasoner")]
    DeepSeekReasoner,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
}

pub struct Client {
    pub model: Model,
    pub thinking: Option<Thinking>,
    pub api_key: String,

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

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or `temperature`` but not both.
    pub top_p: Option<f32>,

    pub context: Vec<request::message::Message>,

    pub tools: Vec<Function>,
}

impl Client {
    #[must_use]
    pub fn new(model: Model, api_key: &str) -> Self {
        Self {
            model,
            thinking: None,
            api_key: api_key.to_string(),
            frequency_penalty: None,
            max_tokens: None,
            presence_penalty: None,
            temperature: None,
            top_p: None,
            context: Vec::new(),
            tools: Vec::new(),
        }
    }

    #[must_use]
    pub async fn chat(&mut self, message: &str) -> Vec<request::message::Message> {
        self.context.push(
            request::message::User {
                name: None,
                content: message.to_string(),
            }
            .into(),
        );

        let start_index = self.context.len();

        loop {
            let client = reqwest::Client::new();
            let resp = client
                .post(format!("{BASE_URL}/chat/completions"))
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .body(
                    serde_json::to_string(&ChatCompletionRequest {
                        model: self.model.clone(),
                        messages: self.context.clone(),
                        thinking: self.thinking.clone(),
                        stream: false,
                        frequency_penalty: self.frequency_penalty,
                        max_tokens: self.max_tokens,
                        presence_penalty: self.presence_penalty,
                        temperature: self.temperature,
                        top_p: self.top_p,
                        tools: self.tools.iter().map(|tool| tool.clone().into()).collect(),
                    })
                    .unwrap(),
                )
                .send()
                .await
                .unwrap();
            let resp: no_streaming::Response = resp.json().await.unwrap();

            assert_eq!(resp.choices.len(), 1);
            let choice = &resp.choices[0];

            self.context.push(
                request::message::Assistant {
                    name: None,
                    content: choice.message.content.to_owned(),
                    reasoning_content: choice.message.reasoning_content.to_owned(),
                    tool_calls: choice.message.tool_calls.clone().map(|tool_calls| {
                        tool_calls
                            .iter()
                            .map(|tool_call| request::message::ToolCall {
                                r#type: tool_call.r#type.clone(),
                                id: tool_call.id.clone(),
                                function: request::message::Function {
                                    name: tool_call.function.name.clone(),
                                    arguments: tool_call.function.arguments.clone(),
                                },
                            })
                            .collect()
                    }),
                }
                .into(),
            );

            if let Some(ref tool_calls) = choice.message.tool_calls {
                for tool_call in tool_calls {
                    let func = self
                        .tools
                        .iter()
                        .find(|tool| tool.name == tool_call.function.name)
                        .unwrap()
                        .call;
                    self.context.push(
                        request::message::Tool {
                            tool_call_id: tool_call.id.clone(),
                            content: func(tool_call.function.arguments.clone()),
                        }
                        .into(),
                    );
                }
            }

            match choice.finish_reason {
                FinishReason::ToolCalls => continue,
                _ => break,
            }
        }

        self.context[start_index..].to_vec()
    }

    #[must_use]
    pub async fn streaming_chat(&mut self, message: &str) -> BoxStream<'_, streaming::Delta> {
        self.context.push(
            request::message::User {
                name: None,
                content: message.to_string(),
            }
            .into(),
        );

        let mut finish_reason: Option<FinishReason> = None;

        let stream = stream! {
            loop {
                let client = reqwest::Client::new();
                let mut resp = client
                    .post(format!("{BASE_URL}/chat/completions"))
                    .header("Content-Type", "application/json")
                    .header("Accept", "application/json")
                    .header("Authorization", format!("Bearer {}", self.api_key))
                    .body(
                        serde_json::to_string(&ChatCompletionRequest {
                            model: self.model.clone(),
                            messages: self.context.clone(),
                            thinking: self.thinking.clone(),
                            stream: true,
                            frequency_penalty: self.frequency_penalty,
                            max_tokens: self.max_tokens,
                            presence_penalty: self.presence_penalty,
                            temperature: self.temperature,
                            top_p: self.top_p,
                            tools: self.tools.iter().map(|tool| tool.clone().into()).collect(),
                        })
                        .unwrap(),
                    )
                    .send()
                    .await
                    .unwrap();

                let mut assistant_msg = request::message::Assistant {
                    name: None,
                    content: String::new(),
                    reasoning_content: None,
                    tool_calls: None,
                };

                let mut tool_calls: Vec<streaming::ToolCall> = Vec::new();

                while let Some(chunk) = resp.chunk().await.unwrap() {
                    let s = String::from_utf8(chunk.to_vec()).unwrap();
                    for data in s.trim().split("\n\n").map(|s| s[6..].to_string()) {
                        if data == "[DONE]" {
                            break;
                        }
                        let chunk: Chunk = serde_json::from_str(&data).unwrap();
                        for choice in chunk.choices {
                            let delta = choice.delta;
                            match delta.clone() {
                                streaming::Delta::Assistant {
                                    content,
                                    reasoning_content,
                                    ..
                                } => {
                                    if let Some(ref content) = content {
                                        assistant_msg.content.push_str(content);
                                    }
                                    if let Some(ref reasoning_content) = reasoning_content {
                                        assistant_msg.reasoning_content.get_or_insert_default().push_str(reasoning_content);
                                    }
                                },
                                streaming::Delta::ToolCall { tool_calls: tool_calls_delta } => {
                                    assert_eq!(tool_calls_delta.len(), 1);
                                    let tool_call_delta = &tool_calls_delta[0];
                                    if tool_call_delta.index == tool_calls.len() {
                                        tool_calls.push(tool_call_delta.clone());
                                    } else {
                                        tool_calls[tool_call_delta.index]
                                            .function
                                            .arguments
                                            .push_str(&tool_call_delta.function.arguments);
                                    }
                                },
                            }

                            if let Some(fr) = choice.finish_reason {
                                finish_reason = Some(fr);
                            }

                            yield delta;
                        }
                    }
                }

                if !tool_calls.is_empty() {
                    assistant_msg.tool_calls = Some(tool_calls.iter().map(|tool_call| request::message::ToolCall {
                        r#type: tool_call.r#type.clone().unwrap(),
                        id: tool_call.id.clone().unwrap(),
                        function: request::message::Function {
                            name: tool_call.function.name.clone().unwrap(),
                            arguments: tool_call.function.arguments.clone()
                        },
                    }).collect());
                }

                self.context.push(assistant_msg.into());

                match finish_reason {
                    Some(FinishReason::ToolCalls) => {
                        for tool_call in tool_calls {
                            let tool_call_id = tool_call.id.unwrap();
                            if self.context.iter().any(|msg| match msg {
                                request::message::Message::Tool(tool) => tool.tool_call_id == tool_call_id,
                                _ => false
                            }) {
                                continue;
                            }
                            let call = self
                                    .tools
                                    .iter()
                                    .find(|tool| tool.name == tool_call.function.name.clone().unwrap())
                                    .unwrap()
                                    .call;
                            self.context.push(request::message::Tool {
                                tool_call_id,
                                content: call(tool_call.function.arguments),
                            }.into());
                        }
                    },
                    None => unreachable!(),
                    _ => break
                }
            }
        };

        stream.boxed()
    }

    #[must_use]
    /// Get user current balance
    pub async fn user_balance(&self) -> UserBalance {
        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{BASE_URL}/user/balance"))
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .unwrap();
        resp.json().await.unwrap()
    }
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Schema,
    pub call: fn(String) -> String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolCallType {
    Function,
}
