# Key Ownership Prover

A secure and efficient key ownership proof system implemented in Rust, using JWT (JSON Web Tokens) and ES256 (ECDSA with P-256 curve) for digital signatures. The system consists of a verifier service and a holder client that demonstrate cryptographic proof of key ownership.

## Project Overview

The system implements a challenge-response protocol where:

1. A holder requests a nonce from the verifier
2. The holder signs the nonce with their private key
3. The verifier validates the signature using the holder's public key

### Components

- **Verifier Service** (`src/main.rs`): A web service that:

  - Generates secure nonces
  - Validates JWT signatures
  - Prevents replay attacks
  - Exposes REST API endpoints

- **Holder Client** (`src/bin/holder.rs`): A client that:
  - Generates ES256 key pairs
  - Requests nonces from the verifier
  - Creates and signs JWTs
  - Demonstrates key ownership

## Prerequisites

- Rust and Cargo (latest stable version)
- Docker and Docker Compose (for containerized deployment)
- OpenSSL development packages

## Running the Project

### Local Development

1. Build the project:

   ```bash
   cargo build
   ```

2. Start the verifier service:

   ```bash
   cargo run --bin verifier
   ```

   The service will start on `http://0.0.0.0:8080`

3. Run the holder client:
   ```bash
   cargo run --bin holder
   ```

### Docker Deployment

1. Build and start the services:
   ```bash
   docker-compose up --build
   ```
   This will start both the verifier and holder services in containers.

## API Endpoints

### Verifier Service

- `GET /api/nonce`

  - Generates a new nonce for signing
  - Returns: `{"nonce": "<uuid-v4>"}`

- `POST /api/verify`

  - Verifies a signed JWT and nonce
  - Request body: `{"jwt": "<signed-token>", "public_key_pem": "<public-key>"}`
  - Returns: `{"verified": boolean, "message": "string"}`

- `GET /api/list-nonces`

  - Lists all used nonces (for debugging)
  - Returns: `{"nonce_count": number, "nonces": ["string"]}`

- `POST /api/clear-nonces`
  - Clears the used nonces store
  - Returns: `{"message": "string"}`

## Security Features

1. **Cryptographic Security**

   - Uses ES256 (ECDSA with P-256 curve and SHA-256)
   - Implements industry-standard JWT format
   - Private keys never leave the holder

2. **Replay Attack Prevention**

   - Tracks used nonces in memory
   - Rejects previously used nonces
   - Implements nonce clearing mechanism

## Dependencies

Key dependencies from `Cargo.toml`:

- `actix-web`: Web framework
- `jwt-simple`: JWT implementation
- `jsonwebtoken`: JWT validation
- `serde`: Serialization
- `uuid`: Nonce generation
- `reqwest`: HTTP client

To generate ES256 (ECDSA with P-256 curve) key pairs locally using OpenSSL, follow these steps:

1. **Generate Private Key**

   ```bash
   # Generate a private key using the P-256 curve
   openssl ecparam -name prime256v1 -genkey -noout -out private.pem
   ```

2. **Extract Public Key**

   ```bash
   # Derive the public key from the private key
   openssl ec -in private.pem -pubout -out public.pem
   ```

3. **Verify Key Format**

   ```bash
   # View the private key content (keep this secure!)
   openssl ec -in private.pem -text -noout

   # View the public key content
   openssl ec -in public.pem -pubin -text -noout
   ```

The generated keys will be in PEM format and can be used directly with the holder client or for manual testing.

## JWT Token Generation with JWT.io

You can use [JWT.io](https://jwt.io) to manually generate and verify JWTs for testing and understanding the token structure:

1. **Header Setup**

   - Algorithm: Select "ES256" (ECDSA with P-256 and SHA-256)
   - The header will look like: `{"alg": "ES256", "typ": "JWT"}`

2. **Payload Configuration**

   - Add the nonce received from the verifier
   - Example payload: `{"nonce": "<received-nonce>", "exp": <170000000>}`

3. **Signature Process**

   - Paste your ES256 private key in PEM format
   - The public key will be used for verification
   - JWT.io will automatically generate the signature

4. **Token Verification**
   - Paste the complete JWT token
   - Input the public key in PEM format
   - JWT.io will verify the signature integrity

## Error Handling

The system implements comprehensive error handling:

- JWT validation errors
- Invalid public key format
- Network communication errors
- Nonce reuse attempts

## Logging

Uses the `env_logger` crate for configurable logging:

- Set `RUST_LOG=info` for standard logs
- Set `RUST_LOG=debug` for detailed debugging

Logs include:

- API request handling
- Nonce generation and validation
- JWT verification results
- Error conditions
