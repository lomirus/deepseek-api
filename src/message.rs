use crate::Role;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct System {
    pub name: Option<String>,
    pub content: String,
}

impl From<System> for Message {
    fn from(value: System) -> Self {
        Message::System(value)
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: Option<String>,
    pub content: String,
}

impl From<User> for Message {
    fn from(value: User) -> Self {
        Message::User(value)
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Tool {
    pub tool_call_id: String,
    pub content: String,
}

impl From<Tool> for Message {
    fn from(value: Tool) -> Self {
        Message::Tool(value)
    }
}

#[derive(Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    pub function: Function,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arguments: String,
}
