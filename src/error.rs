use std::fmt;

/// Application error types
#[derive(Debug)]
pub enum AppError {
    /// Configuration errors
    Config(String),

    /// API related errors
    Api(ApiError),

    /// IO errors
    Io(std::io::Error),

    /// JSON parsing errors
    Json(serde_json::Error),

    /// Cache errors
    Cache(String),

    /// Processing errors
    Processing(String),

    /// Other errors
    Other(Box<dyn std::error::Error>),
}

#[derive(Debug)]
pub enum ApiError {
    /// Missing API key
    MissingApiKey,

    /// API request failed
    RequestFailed { status: u16, message: String },

    /// Rate limit exceeded
    RateLimitExceeded,

    /// Invalid response format
    InvalidResponse(String),

    /// Network error
    Network(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AppError::Api(err) => write!(f, "API error: {}", err),
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Json(err) => write!(f, "JSON parsing error: {}", err),
            AppError::Cache(msg) => write!(f, "Cache error: {}", msg),
            AppError::Processing(msg) => write!(f, "Processing error: {}", msg),
            AppError::Other(err) => write!(f, "Error: {}", err),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::MissingApiKey => {
                write!(f, "OpenAI API key is required. Set OPENAI_API_KEY environment variable or use --api-key option.")
            }
            ApiError::RequestFailed { status, message } => {
                write!(f, "API request failed with status {}: {}", status, message)
            }
            ApiError::RateLimitExceeded => {
                write!(f, "API rate limit exceeded. Please wait before retrying.")
            }
            ApiError::InvalidResponse(msg) => {
                write!(f, "Invalid API response: {}", msg)
            }
            ApiError::Network(msg) => {
                write!(f, "Network error: {}", msg)
            }
        }
    }
}

impl std::error::Error for AppError {}
impl std::error::Error for ApiError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Json(err)
    }
}

impl From<ApiError> for AppError {
    fn from(err: ApiError) -> Self {
        AppError::Api(err)
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::Other(err)
    }
}

/// Result type alias for the application
pub type Result<T> = std::result::Result<T, AppError>;
