use serde::{Deserialize, Serialize};

use crate::api;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Model {
    #[serde(rename = "deepseek-chat")]
    DeepSeekChat,
    #[serde(rename = "deepseek-reasoner")]
    DeepSeekReasoner,
}

#[derive(Clone)]
pub enum ResponseFormat {
    Text,
    JsonObject,
}

impl From<ResponseFormat> for api::request::ResponseFormatType {
    fn from(value: ResponseFormat) -> Self {
        match value {
            ResponseFormat::Text => api::request::ResponseFormatType::Text,
            ResponseFormat::JsonObject => api::request::ResponseFormatType::JsonObject,
        }
    }
}

impl From<ResponseFormat> for api::request::ResponseFormat {
    fn from(value: ResponseFormat) -> Self {
        api::request::ResponseFormatType::from(value).into()
    }
}
