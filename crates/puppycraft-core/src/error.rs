use thiserror::Error;

#[derive(Error, Debug)]
pub enum PuppyCraftError {
    #[error("Store error: {0}")]
    Store(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid input: {0}")]
    Invalid(String),
    #[error("Image error: {0}")]
    Image(String),
    #[error("LLM error: {0}")]
    Llm(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl PuppyCraftError {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Store(_) => "store",
            Self::NotFound(_) => "not_found",
            Self::Invalid(_) => "invalid",
            Self::Image(_) => "image",
            Self::Llm(_) => "llm",
            Self::Serialization(_) => "serialize",
        }
    }
}
