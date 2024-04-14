use async_openai::types::{ChatCompletionToolChoiceOption, CreateChatCompletionRequest};
use navichat::oa_client::new_oa_client;
use navichat::{chat, gpts};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oa_client = new_oa_client()?;
    let chat_client = oa_client.chat();

    let model = gpts::MODEL.to_string();

    let question = "Why is the weather in California's best city and Paris?";

    let messages = vec![chat::user_msg(question)?];

    // build tools
    let tool_weather = chat::tool_fn(
        "get_weather",
        "get the weather for a cty",
        json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA",
                },
                "country": {
                    "type": "string",
                    "description": "The full country name of the city",
                },
                "unit": {
                    "type": "string", "enum": ["celsius", "fahrenheit"],
                    "description": "The full country name of the city",
                },
            },
            "required": ["location", "country", "unit"],
        }),
    )?;
    let tools = Some(vec![tool_weather]);

    let msg_req = CreateChatCompletionRequest {
        model,
        messages,
        tools,
        tool_choice: Some(ChatCompletionToolChoiceOption::Auto),
        ..Default::default()
    };

    let chat_response = chat_client.create(msg_req).await?;
    let first_choice = chat::first_choicel(chat_response)?;

    // extract and print
    if let Some(tool_calls) = first_choice.message.tool_calls {
        for tool in tool_calls {
            println!(
                "function {} args {}",
                tool.function.name, tool.function.arguments
            );
        }
    };

    Ok(())
}
