use rand::{thread_rng, Rng};
use crate::{
    error::Error,
    math::field::FiniteField,
    share::Share,
};

/// Represents the Shamir Secret Sharing scheme.
pub struct ShamirSecretSharing {
    field: FiniteField, // The finite field used for operations
    threshold: usize, // Minimum number of shares needed to reconstruct the secret
    total_shares: usize, // Total number of shares to generate
}

impl ShamirSecretSharing {
    /// Creates a new Shamir Secret Sharing scheme
    pub fn new(threshold: usize, total_shares: usize, prime: u64) -> Result<Self, Error> {
        if threshold == 0 {
            return Err(Error::InvalidThreshold("Threshold must be positive".into()));
        }
        if threshold > total_shares {
            return Err(Error::InvalidThreshold("Threshold must not exceed total shares".into()));
        }

        Ok(ShamirSecretSharing {
            field: FiniteField::new(prime), // Initialize the finite field
            threshold,
            total_shares,
        })
    }

    /// Generates a random coefficient for our polynomial
    fn random_coefficient(&self) -> Result<u64, Error> {
        let mut rng = thread_rng();
        let value = rng.gen_range(0..self.field.prime); // Generate a random value within the field
        Ok(value)
    }

    /// Evaluates the polynomial at point x using Horner's method
    fn evaluate_polynomial(&self, coefficients: &[u64], x: u64) -> u64 {
        let mut result = coefficients[0]; // Start with the constant term
        let mut power = 1u64;
        
        println!("Evaluating polynomial at x={}", x);
        println!("Starting with coefficient[0]={}", coefficients[0]);
        
        for (i, &coeff) in coefficients.iter().skip(1).enumerate() {
            power = self.field.multiply(power, x); // Update power of x
            let term = self.field.multiply(coeff, power); // Calculate term
            result = self.field.add(result, term); // Add term to result
            println!("Step {}: power={}, coeff={}, term={}, result={}", 
                    i+1, power, coeff, term, result);
        }
        
        result // Return the evaluated result
    }

    /// Splits a secret into n shares where k shares are required for reconstruction
    pub fn split_secret(&self, secret: u64) -> Result<Vec<Share>, Error> {
        if secret >= self.field.prime {
            return Err(Error::FieldError("Secret too large for field".into()));
        }

        // Generate random coefficients for polynomial
        let mut coefficients = vec![secret]; // Start with the secret as the first coefficient
        for _ in 1..self.threshold {
            coefficients.push(self.random_coefficient()?); // Generate random coefficients
        }

        // Generate shares by evaluating polynomial at points 1 through n
        Ok((1..=self.total_shares)
            .map(|x| Share::new(
                x as u64,
                self.evaluate_polynomial(&coefficients, x as u64), // Evaluate polynomial for each x
                self.field.prime
            ))
            .collect())
    }

    /// Computes the Lagrange basis polynomial for the given index
    fn lagrange_basis(&self, shares: &[Share], x: u64, idx: usize) -> Option<u64> {
        let mut numerator = 1u64;
        let mut denominator = 1u64;
        
        for (j, share) in shares.iter().enumerate() {
            if j != idx {
                numerator = self.field.multiply(numerator, 
                    self.field.subtract(x, share.x)); // Calculate numerator
                denominator = self.field.multiply(denominator,
                    self.field.subtract(shares[idx].x, share.x)); // Calculate denominator
            }
        }
        
        self.field.divide(numerator, denominator) // Return the Lagrange basis polynomial
    }

    /// Reconstructs the secret from k or more shares using Lagrange interpolation
    pub fn reconstruct_secret(&self, shares: &[Share]) -> Result<u64, Error> {
        if shares.len() < self.threshold {
            return Err(Error::InsufficientShares); // Check if enough shares are provided
        }

        // We only need threshold number of shares
        let shares = &shares[0..self.threshold];
        let mut secret = 0u64;

        // Compute the secret using Lagrange interpolation at x=0
        for (i, share) in shares.iter().enumerate() {
            if let Some(basis) = self.lagrange_basis(shares, 0, i) {
                secret = self.field.add(secret, 
                    self.field.multiply(share.y, basis)); // Add to the secret
            } else {
                return Err(Error::FieldError("Lagrange interpolation failed".into())); // Handle failure
            }
        }

        Ok(secret) // Return the reconstructed secret
    }
}