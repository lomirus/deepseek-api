#![feature(gen_blocks)]
#![feature(async_iterator)]

extern crate self as deepseek_api;

mod client;
mod http;
pub mod message;
mod tool;

use std::{async_iter::AsyncIterator, future::poll_fn, pin::Pin};

use serde::{Deserialize, Serialize};

pub use client::Client;
pub use deepseek_api_macros::tool;
pub use http::response::FinishReason;
pub use tool::{Tool, ToolFuture};

#[doc(hidden)]
pub mod __private {
    pub use schemars;
    pub use serde;
    pub use serde_json;
}

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

#[derive(Debug, Clone)]
pub enum Delta {
    Thinking {
        reasoning_content: String,
        role: Option<Role>,
    },
    Content {
        content: String,
        role: Option<Role>,
    },
    ToolCallInput {
        tool_call_id: Option<String>,
        name: Option<String>,
        arguments: String,
    },
    ToolCallOutput {
        tool_call_id: String,
        content: String,
    },
}

#[derive(Clone)]
pub enum ResponseFormat {
    Text,
    JsonObject,
}

pub trait AsyncIteratorNext: AsyncIterator {
    fn next(&mut self) -> impl std::future::Future<Output = Option<Self::Item>> + Send
    where
        Self: Unpin + Send,
    {
        async { poll_fn(|cx| Pin::new(&mut *self).poll_next(cx)).await }
    }
}

impl<T: AsyncIterator> AsyncIteratorNext for T {}
