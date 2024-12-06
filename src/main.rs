use shamir_sss::ShamirSecretSharing;

fn main() -> Result<(), shamir_sss::Error> {
    let prime = 1001; // Define a prime number for the finite field
    let scheme = ShamirSecretSharing::new(3, 5, prime)?; // Create a new secret sharing scheme
    let secret = 2040 % prime; // Ensure secret is within field
    
    let shares = scheme.split_secret(secret)?; // Split the secret into shares
    let reconstructed = scheme.reconstruct_secret(&shares[0..3])?; // Reconstruct the secret from shares
    
    println!("Original secret: {}", secret); // Print the original secret
    println!("Reconstructed secret: {}", reconstructed); // Print the reconstructed secret
    
    Ok(())
}