/// Main library file for the Shamir Secret Sharing implementation.
pub mod error; // Error handling module
pub mod math; // Mathematical operations module
pub mod share; // Share management module
pub mod scheme; // Secret sharing scheme module

// Re-export commonly used items for easier access
pub use scheme::shamirs::ShamirSecretSharing; // Shamir Secret Sharing struct
pub use share::Share; // Share struct
pub use error::Error; // Error enum