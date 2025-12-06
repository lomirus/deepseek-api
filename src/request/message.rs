
use serde::{Deserialize, Serialize};

use crate::ToolCallType;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "role")]
#[serde(rename_all = "snake_case")]
pub enum Message {
    System(System),
    User(User),
    Assistant(Assistant),
    Tool(Tool),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct System {
    name: Option<String>,
    content: String,
}

impl From<System> for Message {
    fn from(value: System) -> Self {
        Message::System(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: Option<String>,
    pub content: String,
}

impl From<User> for Message {
    fn from(value: User) -> Self {
        Message::User(value)
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Assistant {
    pub name: Option<String>,
    pub content: String,
    pub reasoning_content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>
}

impl From<Assistant> for Message {
    fn from(value: Assistant) -> Self {
        Message::Assistant(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    pub tool_call_id: String,
    pub content: String,
}

impl From<Tool> for Message {
    fn from(value: Tool) -> Self {
        Message::Tool(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub r#type: ToolCallType,
    pub id: String,
    pub function: Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arguments: String,
}
