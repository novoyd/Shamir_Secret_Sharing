#[derive(Debug, Clone, PartialEq)]
pub struct Share {
    pub x: u64, // x coordinate of the share
    pub y: u64, // y coordinate of the share
}

impl Share {
    /// Creates a new Share with given x and y values, ensuring they are within the finite field.
    pub fn new(x: u64, y: u64, prime: u64) -> Self {
        Share {
            x: x % prime, // Ensure x is within the field
            y: y % prime, // Ensure y is within the field
        }
    }

    // Adds two shares with the same x coordinate
    pub fn add(&self, other: &Share, prime: u64) -> Share {
        assert_eq!(self.x, other.x, "Shares must have same x coordinate");
        Share::new(
            self.x, 
            (self.y as u128 + other.y as u128) as u64 % prime, // Add y values
            prime
        )
    }

    // Adds a constant to the y value of the share
    pub fn add_constant(&self, constant: u64, prime: u64) -> Share {
        Share::new(
            self.x,
            (self.y as u128 + constant as u128) as u64 % prime, // Add constant to y
            prime
        )
    }

    // Multiplies the y value of the share by a constant
    pub fn multiply_by_constant(&self, constant: u64, prime: u64) -> Share {
        Share::new(
            self.x,
            (self.y as u128 * constant as u128) as u64 % prime, // Multiply y by constant
            prime
        )
    }
}