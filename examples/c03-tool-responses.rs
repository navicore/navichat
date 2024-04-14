use async_openai::types::{ChatCompletionToolChoiceOption, CreateChatCompletionRequest};
use navichat::oa_client::new_oa_client;
use navichat::{chat, gpts};
use rpc_router::router_builder;
use rpc_router::RpcParams;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize, RpcParams)]
struct GetWeatherParams {
    location: String,
    country: String,
    unit: String,
}

#[derive(Serialize)]
struct Weather {
    temperature: f64,
    unit: String,
    humidity_rh: f32,
}

async fn get_weather(params: GetWeatherParams) -> Result<Weather, String> {
    Ok(Weather {
        temperature: 30.,
        unit: params.unit,
        humidity_rh: 0.3,
    })
}

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

    // init rpc_router
    let rpc_router = router_builder!(get_weather).build();

    let msg_req = CreateChatCompletionRequest {
        model: model.to_string(),
        messages: messages.clone(),
        tools: tools.clone(),
        tool_choice: Some(ChatCompletionToolChoiceOption::Auto),
        ..Default::default()
    };

    let chat_response = chat_client.create(msg_req).await?;
    let first_choice = chat::first_choicel(chat_response)?;

    // if msg content, end early
    if let Some(message_content) = first_choice.message.content {
        println!("Response early - no tools: {message_content}");
        return Ok(());
    };

    // otherwise get / call tools and capture tool responses.

    #[derive(Debug)]
    struct ToolResponse {
        tool_call_id: String,
        response: Value,
    }
    let mut tool_responses: Vec<ToolResponse> = Vec::new();

    let tool_calls = first_choice.message.tool_calls;

    for tool_call in tool_calls.iter().flatten() {
        let tool_call_id = tool_call.id.clone();
        let function_name = tool_call.function.name.clone();
        let params = serde_json::from_str(&tool_call.function.arguments)?;

        let call_result = rpc_router
            .call_route(None, function_name, Some(params))
            .await?;
        let response = call_result.value;
        tool_responses.push(ToolResponse {
            tool_call_id,
            response,
        });
    }

    let mut messages = messages;
    if let Some(tool_call) = tool_calls {
        messages.push(chat::tool_calls_msg(tool_call)?);
    };

    for ToolResponse {
        tool_call_id,
        response,
    } in tool_responses
    {
        messages.push(chat::tool_response_msg(tool_call_id, response)?);
    }

    let msg_req = CreateChatCompletionRequest {
        model: model.to_string(),
        messages,
        tools,
        tool_choice: Some(ChatCompletionToolChoiceOption::Auto),
        ..Default::default()
    };
    let chat_response = chat_client.create(msg_req).await?;
    let first_choice = chat::first_choicel(chat_response)?;
    let content = first_choice.message.content.ok_or("No content.")?;
    println!("Final Response:\n\n{content}");

    Ok(())
}
