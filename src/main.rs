// Importing the Shamir Secret Sharing library
use shamir_sss::ShamirSecretSharing;

fn main() -> Result<(), shamir_sss::Error> {
    // Define a small prime number for the secret sharing scheme
    let prime = 2039; // chosen smaller prime for simpler understanding 
    // Initialize the Shamir Secret Sharing scheme with 3 shares needed to reconstruct from 5 total shares
    let scheme = ShamirSecretSharing::new(3, 5, prime)?;
    
    println!("Demonstrating Shamir Secret Sharing Scheme\n");

    // Basic Secret Sharing
    println!("1. Basic Secret Sharing:");
    let secret = 1234; // The secret to be shared
    // Split the secret into shares
    let shares = scheme.split_secret(secret)?;
    
    // Display the original secret and the generated shares
    println!("Original secret: {}", secret);
    println!("Generated shares:");
    for (i, share) in shares.iter().enumerate() {
        // Print each share's x and y values
        println!("Share {}: ({}, {})", i+1, share.x, share.y);
    }

    // Reconstruct the secret using the first three shares
    let reconstructed = scheme.reconstruct_secret(&shares[0..3])?;
    println!("Reconstructed secret using 3 shares: {}\n", reconstructed);

    // Share Addition
    println!("2. Share Addition:");
    let secret1 = 20; // First secret
    let secret2 = 30; // Second secret
    // Split both secrets into shares
    let shares1 = scheme.split_secret(secret1)?;
    let shares2 = scheme.split_secret(secret2)?;
    
    // Add the first shares of both secrets
    let sum_share = shares1[0].add(&shares2[0], prime);
    println!("First secret: {}", secret1);
    println!("Second secret: {}", secret2);
    println!("Sum of secrets should be: {}", secret1 + secret2);
    // Display the result of the share addition
    println!("Share after addition: ({}, {})\n", sum_share.x, sum_share.y);

    // Addition with Public Value
    println!("3. Adding Public Value to Share:");
    let public_value = 15; // Public value to be added
    // Add the public value to the first share of the first secret
    let share_plus_public = shares1[0].add_constant(public_value, prime);
    println!("Original share: ({}, {})", shares1[0].x, shares1[0].y);
    println!("Public value: {}", public_value);
    // Display the result after adding the public value
    println!("Share after adding public value: ({}, {})\n", 
             share_plus_public.x, share_plus_public.y);

    // Multiplication with Public Value
    println!("4. Multiplying Share with Public Value:");
    let public_multiplier = 3; // Public multiplier
    // Multiply the first share of the first secret by the public multiplier
    let share_times_public = shares1[0].multiply_by_constant(public_multiplier, prime);
    println!("Original share: ({}, {})", shares1[0].x, shares1[0].y);
    println!("Public multiplier: {}", public_multiplier);
    // Display the result after multiplication
    println!("Share after multiplication: ({}, {})", 
             share_times_public.x, share_times_public.y);

    Ok(())
}