use std::fmt::Display;

use crate::Result;
use async_openai::types::{
    ChatChoice, ChatCompletionMessageToolCall, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestMessage, ChatCompletionRequestToolMessageArgs,
    ChatCompletionRequestUserMessageArgs, ChatCompletionTool, ChatCompletionToolArgs,
    CreateChatCompletionResponse, FunctionObject,
};
use serde_json::Value;

/// Create a message with user content
pub fn user_msg(content: impl Into<String>) -> Result<ChatCompletionRequestMessage> {
    let msg = ChatCompletionRequestUserMessageArgs::default()
        .content(content.into())
        .build()?;

    Ok(msg.into())
}

/// Create a message with a tool call
pub fn tool_calls_msg(
    tool_calls: Vec<ChatCompletionMessageToolCall>,
) -> Result<ChatCompletionRequestMessage> {
    let msg = ChatCompletionRequestAssistantMessageArgs::default()
        .tool_calls(tool_calls)
        .build()?;
    Ok(msg.into())
}

/// Create a tool call
pub fn tool_fn(
    name: impl Into<String>,
    description: impl Into<String>,
    parameters: Value,
) -> Result<ChatCompletionTool> {
    let tool = ChatCompletionToolArgs::default()
        .function(FunctionObject {
            name: name.into(),
            description: Some(description.into()),
            parameters: Some(parameters),
            strict: None,
        })
        .build()?;
    Ok(tool)
}

/// Get the first choice from a chat response
pub fn first_choicel(chat_response: CreateChatCompletionResponse) -> Result<ChatChoice> {
    let first_choice = chat_response
        .choices
        .into_iter()
        .next()
        .ok_or("no first choice?")?;
    Ok(first_choice)
}

pub fn tool_response_msg(
    tool_call_id: String,
    content: impl Display,
) -> Result<ChatCompletionRequestMessage> {
    let msg = ChatCompletionRequestToolMessageArgs::default()
        .tool_call_id(tool_call_id)
        .content(content.to_string())
        .build()?;
    Ok(msg.into())
}

// unit tests for the chat module
//
// cargo test --lib chat
pub mod tests {

    #[test]
    fn test_user_msg() {
        use super::user_msg;
        let msg = user_msg("hello");
        assert!(msg.is_ok());
    }
}
