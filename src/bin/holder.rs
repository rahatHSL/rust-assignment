use jwt_simple::prelude::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct NonceResponse {
    nonce: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VerificationRequest {
    jwt: String,
    public_key_pem: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VerificationResponse {
    verified: bool,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    nonce: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Key Ownership Prover - Holder");
    println!("================================\n");

    // Generate a new ES256 key pair
    println!("Generating a new ES256 key pair...");
    let key_pair = ES256KeyPair::generate();
    let public_key = key_pair.public_key();
    
    // Export the public key to PEM format
    let public_key_pem = public_key.to_pem()?;
    
    println!("Key pair generated successfully!\n");
    
    // Create HTTP client
    let client = Client::new();
    
    // Step 1: Get a nonce from the verifier
    println!("Requesting a nonce from the verifier...");
    let nonce_response = client.get("http://verifier:8080/api/nonce")
        .send()?
        .json::<NonceResponse>()?;
    
    println!("Received nonce: {}\n", nonce_response.nonce);
    
    // Step 2: Create JWT claims with the nonce
    let claims = JwtClaims {
        nonce: nonce_response.nonce.clone(),
    };
    
    // Create token header and claims
    let claims_builder = Claims::with_custom_claims(claims, Duration::from_secs(300));
    
    // Sign the JWT with our private key
    println!("Signing the nonce with our private key...");
    let jwt = key_pair.sign(claims_builder)?;
    
    println!("JWT created successfully!\n");
    println!("JWT: {}", jwt);
    
    // Step 3: Send the JWT and public key to the verifier
    println!("Sending verification request to the verifier...");
    let verification_request = VerificationRequest {
        jwt,
        public_key_pem,
    };
    
    let verification_response = client.post("http://verifier:8080/api/verify")
        .json(&verification_request)
        .send()?
        .json::<VerificationResponse>()?;
    
    println!("\nVerification result: {}", if verification_response.verified { "SUCCESS" } else { "FAILED" });
    println!("Message: {}", verification_response.message);
    
    Ok(())
}