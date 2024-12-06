use super::Share;

/// Trait defining operations that can be performed on shares.
pub trait ShareOperations {
    fn combine_shares(shares: &[Share], prime: u64) -> Option<Share>; // Combine shares
    fn multiply_shares(share1: &Share, share2: &Share, prime: u64) -> Option<Share>; // Multiply shares
}

impl ShareOperations for Share {
    // Combines multiple shares that have the same x coordinate
    fn combine_shares(shares: &[Share], prime: u64) -> Option<Share> {
        if shares.is_empty() {
            return None; // Return None if no shares are provided
        }

        let x = shares[0].x; // Get the x coordinate from the first share
        if !shares.iter().all(|s| s.x == x) {
            return None; // Ensure all shares have the same x coordinate
        }

        let mut combined_y = 0u64; // Initialize combined y value
        for share in shares {
            combined_y = (combined_y + share.y) % prime; // Combine y values
        }

        Some(Share::new(x, combined_y, prime)) // Return the combined share
    }

    // Multiplies two shares (useful for advanced protocols)
    fn multiply_shares(share1: &Share, share2: &Share, prime: u64) -> Option<Share> {
        if share1.x != share2.x {
            return None; // Return None if shares have different x coordinates
        }

        Some(Share::new(
            share1.x,
            ((share1.y as u128 * share2.y as u128) % prime as u128) as u64, // Multiply y values
            prime
        ))
    }
}