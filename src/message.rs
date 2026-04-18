use serde::{Deserialize, Serialize};

use crate::{Role, api};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Message {
    System(System),
    User(User),
    Assistant(Assistant),
    Tool(Tool),
}

impl Message {
    pub fn system(content: &str) -> Self {
        Self::System(System {
            name: None,
            content: content.to_string(),
        })
    }

    pub fn user(content: &str) -> Self {
        Self::User(User {
            name: None,
            content: content.to_string(),
        })
    }

    pub fn role(&self) -> Role {
        match self {
            Self::System(_) => Role::System,
            Self::User(_) => Role::User,
            Self::Assistant(_) => Role::Assistant,
            Self::Tool(_) => Role::Tool,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct System {
    pub name: Option<String>,
    pub content: String,
}

impl From<System> for Message {
    fn from(value: System) -> Self {
        Message::System(value)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub name: Option<String>,
    pub content: String,
}

impl From<User> for Message {
    fn from(value: User) -> Self {
        Message::User(value)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Assistant {
    pub name: Option<String>,
    pub content: String,
    pub reasoning_content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl From<Assistant> for Message {
    fn from(value: Assistant) -> Self {
        Message::Assistant(value)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tool {
    pub tool_call_id: String,
    pub content: String,
}

impl From<Tool> for Message {
    fn from(value: Tool) -> Self {
        Message::Tool(value)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolCall {
    pub id: String,
    pub function: Function,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: String,
}

impl From<&Message> for api::request::Message {
    fn from(value: &Message) -> Self {
        match value {
            Message::System(s) => api::request::Message::System {
                name: s.name.clone(),
                content: s.content.clone(),
            },
            Message::User(u) => api::request::Message::User {
                name: u.name.clone(),
                content: u.content.clone(),
            },
            Message::Assistant(a) => api::request::Message::Assistant {
                name: a.name.clone(),
                content: a.content.clone(),
                reasoning_content: a.reasoning_content.clone(),
                tool_calls: a
                    .tool_calls
                    .as_ref()
                    .map(|tcs| tcs.iter().map(api::request::ToolCall::from).collect()),
            },
            Message::Tool(t) => api::request::Message::Tool {
                tool_call_id: t.tool_call_id.clone(),
                content: t.content.clone(),
            },
        }
    }
}

impl From<&ToolCall> for api::request::ToolCall {
    fn from(value: &ToolCall) -> Self {
        Self {
            r#type: api::ToolCallType::Function,
            id: value.id.clone(),
            function: api::request::Function {
                name: value.function.name.clone(),
                arguments: value.function.arguments.clone(),
            },
        }
    }
}
