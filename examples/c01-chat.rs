use async_openai::types::CreateChatCompletionRequest;
use navichat::oa_client::new_oa_client;
use navichat::{chat, gpts};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oa_client = new_oa_client()?;
    let chat_client = oa_client.chat();

    let model = gpts::MODEL.to_string();

    let question = "Why is the sky red? (be concise)";
    let messages = vec![chat::user_msg(question)?];

    let msg_req = CreateChatCompletionRequest {
        model,
        messages,
        ..Default::default()
    };

    let chat_response = chat_client.create(msg_req).await?;
    let first_choice = chat::first_choicel(chat_response)?;

    let response = first_choice.message.content.ok_or("No message content.")?;

    println!("Result:\n\n{response}");

    Ok(())
}
