# Key Ownership Prover

This project demonstrates a simple key ownership proof system using JWT (JSON Web Tokens) and JWK (JSON Web Keys). It consists of two components:

1. **Verifier Service**: A web service that generates nonces and verifies attestations
2. **Holder Script**: A client that generates a key pair, signs a nonce with its private key, and sends the signature to the verifier

## Prerequisites

- Rust and Cargo installed
- Internet connection for dependency downloads

## How to Run

### Step 1: Build the Project

```bash
cargo build
```

### Step 2: Start the Verifier Service

In one terminal window, run:

```bash
cargo run --bin verifier
```

This will start the verifier service on http://127.0.0.1:8080.

### Step 3: Run the Holder Script

In another terminal window, run:

```bash
cargo run --bin holder
```

The holder script will:
1. Generate a new ES256 key pair
2. Request a nonce from the verifier
3. Sign the nonce with the private key
4. Send the signature and public key to the verifier for verification

## How It Works

1. The holder requests a nonce from the verifier
2. The verifier generates a random nonce and sends it to the holder
3. The holder signs the nonce with its private key and creates a JWT
4. The holder sends the JWT and its public key (in JWK format) to the verifier
5. The verifier validates the JWT signature using the provided public key
6. The verifier checks if the nonce has been used before (to prevent replay attacks)
7. The verifier returns the verification result

## Security Features

- Uses ES256 (ECDSA with P-256 curve and SHA-256) for digital signatures
- Prevents replay attacks by tracking used nonces
- Private key never leaves the holder
- Uses standard JWT and JWK formats

## API Endpoints

- `GET /api/nonce`: Generates a new nonce
- `POST /api/verify`: Verifies a JWT signature and checks the nonce