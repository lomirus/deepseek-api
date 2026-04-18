use crate::Role;

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
