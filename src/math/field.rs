/// Represents a finite field defined by a prime number.
pub struct FiniteField {
    pub prime: u64, // The prime number defining the field
}

impl FiniteField {
    /// Creates a new FiniteField with the given prime number.
    pub fn new(prime: u64) -> Self {
        FiniteField { prime }
    }

    // Addition operation in the finite field
    pub fn add(&self, a: u64, b: u64) -> u64 {
        ((a as u128 + b as u128) % self.prime as u128) as u64
    }

    // Subtraction operation in the finite field
    pub fn subtract(&self, a: u64, b: u64) -> u64 {
        let result = if a >= b {
            a - b
        } else {
            self.prime - ((b - a) % self.prime)
        };
        result % self.prime
    }

    // Multiplication operation in the finite field
    pub fn multiply(&self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.prime as u128) as u64
    }

    // Division operation in the finite field, using modular inverse
    pub fn divide(&self, a: u64, b: u64) -> Option<u64> {
        self.mod_inverse(b).map(|b_inv| self.multiply(a, b_inv))
    }

    // Computes the modular inverse of a number in the finite field
    pub fn mod_inverse(&self, a: u64) -> Option<u64> {
        let mut t = 0i64;
        let mut newt = 1i64;
        let mut r = self.prime as i64;
        let mut newr = a as i64;

        while newr != 0 {
            let quotient = r / newr;
            (t, newt) = (newt, t - quotient * newt);
            (r, newr) = (newr, r - quotient * newr);
        }

        if r > 1 {
            return None; // a is not invertible
        }

        if t < 0 {
            t += self.prime as i64; // Ensure positive result
        } 
        Some(t as u64)
    }
}