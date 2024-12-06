// Organizes all math operations related to the Shamir Secret Sharing implementation.
pub mod field; // Finite field operations
mod polynomial; // Polynomial operations
mod interpolation; // Interpolation operations

pub use field::FiniteField; // Re-export FiniteField for easier access
pub use polynomial::Polynomial; // Re-export Polynomial for easier access
pub use interpolation::lagrange_interpolate; // Re-export Lagrange interpolation function