use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use log::{debug, error, info};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Mutex;
use uuid::Uuid;

// Store for used nonces to prevent replay attacks
static USED_NONCES: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

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
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}

async fn generate_nonce() -> impl Responder {
    let nonce = Uuid::new_v4().to_string();
    debug!("Generated nonce: {}", nonce);

    HttpResponse::Ok().json(NonceResponse { nonce })
}

async fn verify_attestation(req: web::Json<VerificationRequest>) -> impl Responder {
    info!("Received verification request with JWT: {}", req.jwt);

    // Use from_ec_pem for EC keys
    let decoding_key = match DecodingKey::from_ec_pem(req.public_key_pem.as_bytes()) {
        Ok(key) => key,
        Err(e) => {
            error!("Failed to create decoding key from PEM: {}", e);
            return HttpResponse::BadRequest().json(VerificationResponse {
                verified: false,
                message: format!("Invalid public key format: {}", e),
            });
        }
    };

    let validation = Validation::new(Algorithm::ES256);

    let token_data = decode::<JwtClaims>(&req.jwt, &decoding_key, &validation);

    match token_data {
        Ok(data) => {
            let nonce = data.claims.nonce.trim();
            if nonce.is_empty() {
                error!("Nonce extracted is empty");
                return HttpResponse::BadRequest().json(VerificationResponse {
                    verified: false,
                    message: "Invalid nonce extracted from JWT".to_string(),
                });
            }

            let mut used_nonces = USED_NONCES.lock().unwrap();
            info!("Checking nonce: {}", nonce);
            if used_nonces.contains(nonce) {
                info!("Nonce has already been used: {}", nonce);
                return HttpResponse::BadRequest().json(VerificationResponse {
                    verified: false,
                    message: "Nonce has already been used".to_string(),
                });
            }

            used_nonces.insert(nonce.to_string());
            info!("Marked nonce as used: {}", nonce);

            info!("Successfully verified attestation with custom claims");
            HttpResponse::Ok().json(VerificationResponse {
                verified: true,
                message: "Attestation verified successfully".to_string(),
            })
        }
        Err(e) => {
            error!("JWT verification failed: {}", e);
            HttpResponse::BadRequest().json(VerificationResponse {
                verified: false,
                message: format!("JWT verification failed: {}", e),
            })
        }
    }
}

async fn clear_nonces() -> impl Responder {
    let mut used_nonces = USED_NONCES.lock().unwrap();
    let count = used_nonces.len();
    used_nonces.clear();
    info!("Cleared {} used nonces", count);

    HttpResponse::Ok().json(serde_json::json!({
        "message": format!("Cleared {} used nonces", count)
    }))
}

async fn list_nonces() -> impl Responder {
    let used_nonces = USED_NONCES.lock().unwrap();
    let nonce_list: Vec<&String> = used_nonces.iter().collect();

    info!("Current nonces in system: {:?}", nonce_list);

    HttpResponse::Ok().json(serde_json::json!({
        "nonce_count": nonce_list.len(),
        "nonces": nonce_list
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    info!("Starting verifier service on 0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .route("/api/nonce", web::get().to(generate_nonce))
            .route("/api/verify", web::post().to(verify_attestation))
            .route("/api/clear-nonces", web::post().to(clear_nonces))
            .route("/api/list-nonces", web::get().to(list_nonces))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
