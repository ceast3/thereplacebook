//! Custom error types for The Replacebook application.
//!
//! This module defines a comprehensive error handling system using the
//! `thiserror` crate for automatic error trait implementations.

use thiserror::Error;

/// Main error type for the application.
/// 
/// This enum encompasses all possible errors that can occur during
/// the application's execution, from database errors to API failures.
#[derive(Error, Debug)]
pub enum AppError {
    /// Database-related errors from SQLx.
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    /// HTTP request errors from reqwest.
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),
    
    /// JSON parsing errors from serde_json.
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// Indicates a data source is temporarily or permanently unavailable.
    #[error("Data source unavailable: {0}")]
    DataSourceUnavailable(String),
    
    /// Indicates API rate limits have been exceeded.
    #[error("Rate limit exceeded for {0}")]
    RateLimitExceeded(String),
    
    /// Configuration-related errors.
    #[error("Invalid configuration: {0}")]
    Configuration(String),
    
    /// Data validation errors.
    #[error("Data validation error: {0}")]
    Validation(String),
}

/// Convenience type alias for Results with our custom error type.
/// 
/// # Example
/// ```
/// use crate::errors::Result;
/// 
/// fn fetch_data() -> Result<String> {
///     Ok("data".to_string())
/// }
/// ```
pub type Result<T> = std::result::Result<T, AppError>;