#![feature(gen_blocks)]
#![feature(async_iterator)]

extern crate self as deepseek_api;

mod api;
mod client;
mod config;
mod delta;
pub mod message;
mod stream;
mod tool;

use serde::{Deserialize, Serialize};

pub use api::response::FinishReason;
pub use client::Client;
pub use config::{Model, ResponseFormat};
pub use deepseek_api_macros::tool;
pub use delta::Delta;
pub use stream::AsyncIteratorNext;
pub use tool::{Tool, ToolFuture};

#[doc(hidden)]
pub mod __private {
    pub use schemars;
    pub use serde;
    pub use serde_json;
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
