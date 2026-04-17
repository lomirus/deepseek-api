#![feature(gen_blocks)]
#![feature(async_iterator)]

extern crate self as deepseek_api;

mod client;
mod http;

use std::{
    async_iter::AsyncIterator,
    future::{Future, poll_fn},
    pin::Pin,
};

use schemars::Schema;
use serde::{Deserialize, Serialize};

use crate::http::response::streaming;

pub use client::Client;
pub use deepseek_api_macros::tool;
pub use http::request::message;
pub use http::response::FinishReason;

#[doc(hidden)]
pub mod __private {
    pub use schemars;
    pub use serde;
    pub use serde_json;
}

pub type ToolFuture = Pin<Box<dyn Future<Output = String> + Send + 'static>>;

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

#[derive(Clone, Copy)]
pub struct Tool {
    pub(crate) name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) parameters: fn() -> &'static Schema,
    pub(crate) call: fn(String) -> ToolFuture,
}

impl Tool {
    #[doc(hidden)]
    #[must_use]
    pub const fn new(
        name: &'static str,
        description: &'static str,
        parameters: fn() -> &'static Schema,
        call: fn(String) -> ToolFuture,
    ) -> Self {
        Self {
            name,
            description,
            parameters,
            call,
        }
    }
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
        function: streaming::Function,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tool]
    /// Adds two integers.
    async fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[tool]
    /// First line.
    /// Second line.
    async fn multiline() -> &'static str {
        "ok"
    }

    #[tool]
    /// Returns a constant.
    async fn no_args() -> i32 {
        42
    }

    #[tokio::test]
    async fn name_and_description_are_set() {
        assert_eq!(ADD.name, "add");
        assert_eq!(ADD.description, "Adds two integers.");
    }

    #[tokio::test]
    async fn call_invokes_original_function() {
        let result = (ADD.call)(r#"{"a":3,"b":4}"#.to_string()).await;
        assert_eq!(result, "7");
    }

    #[tokio::test]
    async fn multiline_doc_strips_per_line_leading_space() {
        assert_eq!(MULTILINE.description, "First line.\nSecond line.");
    }

    #[tokio::test]
    async fn no_args_function_accepts_empty_object() {
        assert_eq!(NO_ARGS.name, "no_args");
        let result = (NO_ARGS.call)("{}".to_string()).await;
        assert_eq!(result, "42");
    }

    #[tokio::test]
    async fn original_function_is_still_callable() {
        assert_eq!(add(1, 2).await, 3);
    }

    #[test]
    fn parameters_schema_lists_fields() {
        let schema = serde_json::to_value((ADD.parameters)()).unwrap();
        let props = schema.get("properties").expect("schema has properties");
        assert!(props.get("a").is_some());
        assert!(props.get("b").is_some());
    }

    #[test]
    fn tool_const_is_copy() {
        let a = ADD;
        let b = ADD;
        assert_eq!(a.name, b.name);
    }
}
