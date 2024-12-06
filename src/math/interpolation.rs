use super::field::FiniteField;
use crate::share::Share;

/// Performs Lagrange interpolation to reconstruct a value at x from given points.
pub fn lagrange_interpolate(field: &FiniteField, points: &[Share], x: u64) -> Option<u64> {
    let mut result = 0u64;

    // Iterate through each point to compute the Lagrange basis polynomials
    for i in 0..points.len() {
        let mut term = points[i].y; // Start with the y-value of the current point
        
        for j in 0..points.len() {
            if i != j {
                // Calculate numerator (x - x_j)
                let numerator = field.subtract(x, points[j].x);
                
                // Calculate denominator (x_i - x_j)
                let denominator = field.subtract(points[i].x, points[j].x);
                
                // Get multiplicative inverse of denominator
                if let Some(inv_denominator) = field.mod_inverse(denominator) {
                    term = field.multiply(term, 
                           field.multiply(numerator, inv_denominator));
                } else {
                    return None; // Denominator is zero, cannot proceed
                }
            }
        }
        
        result = field.add(result, term); // Add the term to the result
    }

    Some(result) // Return the final interpolated result
}