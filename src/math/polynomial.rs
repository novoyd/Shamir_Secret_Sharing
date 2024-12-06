use super::field::FiniteField;

/// Represents a polynomial defined by its coefficients in a finite field.
pub struct Polynomial {
    coefficients: Vec<u64>, // Coefficients of the polynomial
    field: FiniteField, // The finite field used for operations
}

impl Polynomial {
    /// Creates a new Polynomial with given coefficients and finite field.
    pub fn new(coefficients: Vec<u64>, field: FiniteField) -> Self {
        Polynomial {
            coefficients, 
            field,
        }
    }

    /// Evaluates the polynomial at a given x value.
    pub fn evalute(&self, x: u64) -> u64 {
        let mut result = *self.coefficients.last().unwrap_or(&0); // Start with the last coefficient

        for &coeff in self.coefficients.iter().rev().skip(1) {
            result = self.field.multiply(result, x); // Update result with x
            result = self.field.add(result, coeff); // Add coefficient to result
        }
        result // Return the evaluated result
    }
}