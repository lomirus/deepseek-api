use std::{future::Future, pin::Pin};

use schemars::Schema;

pub type ToolFuture = Pin<Box<dyn Future<Output = String> + Send + 'static>>;

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

#[cfg(test)]
mod tests {
    use crate::tool;

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
