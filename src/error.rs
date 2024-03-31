use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    #[from]
    OpenAi(async_openai::error::OpenAIError),

    #[from]
    Json(serde_json::Error),

    #[from]
    RpcCall(rpc_router::CallError),
}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}
