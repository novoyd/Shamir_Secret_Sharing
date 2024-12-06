use thiserror::Error;

/// Defines custom error types for the Shamir Secret Sharing implementation.
#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid threshold: {0}")]
    InvalidThreshold(String), // Error for invalid threshold
    #[error("Invalid share count: {0}")]
    InvalidShareCountS(String), // Error for invalid share count
    #[error("Field arithmetic error: {0}")]
    FieldError(String), // Error for field arithmetic issues
    #[error("Insufficient shares for reconstruction")]
    InsufficientShares, // Error for insufficient shares
    #[error("Random number generation failed")]
    RngError, // Error for RNG failures
}