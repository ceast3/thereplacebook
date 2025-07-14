use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Data source unavailable: {0}")]
    DataSourceUnavailable(String),
    
    #[error("Rate limit exceeded for {0}")]
    RateLimitExceeded(String),
    
    #[error("Invalid configuration: {0}")]
    Configuration(String),
    
    #[error("Data validation error: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, AppError>;