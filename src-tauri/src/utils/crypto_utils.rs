use crate::error::{HoverShellError, Result};
use log::{error, info};
use std::collections::HashMap;

pub fn encrypt_data(data: &str, key: &str) -> Result<String> {
    use aes_gcm::{Aes256Gcm, Key, Nonce};
    use aes_gcm::aead::{Aead, NewAead};
    
    let key_bytes = sha2::Sha256::digest(key.as_bytes());
    let key = Key::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    
    let nonce_bytes = generate_random_bytes(12);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher.encrypt(nonce, data.as_bytes())
        .map_err(|e| HoverShellError::Security(format!("Encryption failed: {}", e)))?;
    
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    
    Ok(base64::encode(result))
}

pub fn decrypt_data(encrypted_data: &str, key: &str) -> Result<String> {
    use aes_gcm::{Aes256Gcm, Key, Nonce};
    use aes_gcm::aead::{Aead, NewAead};
    
    let data = base64::decode(encrypted_data)
        .map_err(|e| HoverShellError::Security(format!("Base64 decode failed: {}", e)))?;
    
    if data.len() < 12 {
        return Err(HoverShellError::Security("Invalid encrypted data".to_string()));
    }
    
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let key_bytes = sha2::Sha256::digest(key.as_bytes());
    let key = Key::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);
    
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| HoverShellError::Security(format!("Decryption failed: {}", e)))?;
    
    String::from_utf8(plaintext)
        .map_err(|e| HoverShellError::Security(format!("UTF-8 decode failed: {}", e)))
}

pub fn hash_password(password: &str, salt: Option<&str>) -> Result<String> {
    use argon2::{Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
    use argon2::password_hash::{rand_core::OsRng, SaltString};
    
    let salt = if let Some(salt_str) = salt {
        SaltString::from_b64(salt_str)
            .map_err(|e| HoverShellError::Security(format!("Invalid salt: {}", e)))?
    } else {
        SaltString::generate(&mut OsRng)
    };
    
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|e| HoverShellError::Security(format!("Password hashing failed: {}", e)))?;
    
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| HoverShellError::Security(format!("Invalid hash: {}", e)))?;
    
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen()).collect()
}

pub fn generate_random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn generate_api_key() -> String {
    format!("hs_{}", generate_random_string(32))
}

pub fn generate_session_token() -> String {
    format!("session_{}", generate_random_string(64))
}

pub fn hash_string(input: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_file_content(content: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}

pub fn create_hmac(message: &str, key: &str) -> Result<String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    
    type HmacSha256 = Hmac<Sha256>;
    
    let mut mac = HmacSha256::new_from_slice(key.as_bytes())
        .map_err(|e| HoverShellError::Security(format!("HMAC creation failed: {}", e)))?;
    
    mac.update(message.as_bytes());
    let result = mac.finalize();
    
    Ok(base64::encode(result.into_bytes()))
}

pub fn verify_hmac(message: &str, key: &str, signature: &str) -> Result<bool> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    
    type HmacSha256 = Hmac<Sha256>;
    
    let expected_signature = create_hmac(message, key)?;
    Ok(expected_signature == signature)
}

pub fn create_jwt_token(payload: &serde_json::Value, secret: &str) -> Result<String> {
    use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
    
    let header = Header::new(Algorithm::HS256);
    let key = EncodingKey::from_secret(secret.as_bytes());
    
    encode(&header, payload, &key)
        .map_err(|e| HoverShellError::Security(format!("JWT encoding failed: {}", e)))
}

pub fn verify_jwt_token(token: &str, secret: &str) -> Result<serde_json::Value> {
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    
    let token_data = decode::<serde_json::Value>(token, &key, &validation)
        .map_err(|e| HoverShellError::Security(format!("JWT verification failed: {}", e)))?;
    
    Ok(token_data.claims)
}

pub fn generate_rsa_keypair() -> Result<(String, String)> {
    use rsa::{RsaPrivateKey, RsaPublicKey};
    use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
    
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 2048)
        .map_err(|e| HoverShellError::Security(format!("RSA key generation failed: {}", e)))?;
    
    let public_key = RsaPublicKey::from(&private_key);
    
    let private_pem = private_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
        .map_err(|e| HoverShellError::Security(format!("Private key encoding failed: {}", e)))?;
    
    let public_pem = public_key.to_public_key_pem(rsa::pkcs8::LineEnding::LF)
        .map_err(|e| HoverShellError::Security(format!("Public key encoding failed: {}", e)))?;
    
    Ok((private_pem.to_string(), public_pem))
}

pub fn encrypt_with_rsa(data: &str, public_key: &str) -> Result<String> {
    use rsa::{RsaPublicKey, pkcs8::DecodePublicKey};
    use rsa::pkcs1v15::Pkcs1v15Encrypt;
    
    let public_key = RsaPublicKey::from_public_key_pem(public_key)
        .map_err(|e| HoverShellError::Security(format!("Public key parsing failed: {}", e)))?;
    
    let mut rng = rand::thread_rng();
    let encrypted = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data.as_bytes())
        .map_err(|e| HoverShellError::Security(format!("RSA encryption failed: {}", e)))?;
    
    Ok(base64::encode(encrypted))
}

pub fn decrypt_with_rsa(encrypted_data: &str, private_key: &str) -> Result<String> {
    use rsa::{RsaPrivateKey, pkcs8::DecodePrivateKey};
    use rsa::pkcs1v15::Pkcs1v15Encrypt;
    
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_key)
        .map_err(|e| HoverShellError::Security(format!("Private key parsing failed: {}", e)))?;
    
    let data = base64::decode(encrypted_data)
        .map_err(|e| HoverShellError::Security(format!("Base64 decode failed: {}", e)))?;
    
    let decrypted = private_key.decrypt(Pkcs1v15Encrypt, &data)
        .map_err(|e| HoverShellError::Security(format!("RSA decryption failed: {}", e)))?;
    
    String::from_utf8(decrypted)
        .map_err(|e| HoverShellError::Security(format!("UTF-8 decode failed: {}", e)))
}

pub fn create_digital_signature(data: &str, private_key: &str) -> Result<String> {
    use rsa::{RsaPrivateKey, pkcs8::DecodePrivateKey};
    use rsa::pkcs1v15::{SigningKey, Signature};
    use sha2::{Sha256, Digest};
    
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_key)
        .map_err(|e| HoverShellError::Security(format!("Private key parsing failed: {}", e)))?;
    
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let mut rng = rand::thread_rng();
    
    let signature = signing_key.sign_with_rng(&mut rng, data.as_bytes());
    
    Ok(base64::encode(signature.to_bytes()))
}

pub fn verify_digital_signature(data: &str, signature: &str, public_key: &str) -> Result<bool> {
    use rsa::{RsaPublicKey, pkcs8::DecodePublicKey};
    use rsa::pkcs1v15::{VerifyingKey, Signature};
    use sha2::{Sha256, Digest};
    
    let public_key = RsaPublicKey::from_public_key_pem(public_key)
        .map_err(|e| HoverShellError::Security(format!("Public key parsing failed: {}", e)))?;
    
    let verifying_key = VerifyingKey::<Sha256>::new(public_key);
    
    let signature_bytes = base64::decode(signature)
        .map_err(|e| HoverShellError::Security(format!("Base64 decode failed: {}", e)))?;
    
    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|e| HoverShellError::Security(format!("Signature parsing failed: {}", e)))?;
    
    Ok(verifying_key.verify(data.as_bytes(), &signature).is_ok())
}

pub fn generate_otp_secret() -> String {
    use base32::Alphabet;
    let secret = generate_random_bytes(20);
    base32::encode(Alphabet::RFC4648 { padding: true }, &secret)
}

pub fn generate_otp_code(secret: &str, timestamp: u64) -> Result<String> {
    use hmac::{Hmac, Mac};
    use sha1::Sha1;
    
    type HmacSha1 = Hmac<Sha1>;
    
    let secret_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: true }, secret)
        .ok_or_else(|| HoverShellError::Security("Invalid secret".to_string()))?;
    
    let mut mac = HmacSha1::new_from_slice(&secret_bytes)
        .map_err(|e| HoverShellError::Security(format!("HMAC creation failed: {}", e)))?;
    
    let time_step = timestamp / 30;
    let time_bytes = time_step.to_be_bytes();
    
    mac.update(&time_bytes);
    let result = mac.finalize();
    
    let hash = result.into_bytes();
    let offset = (hash[19] & 0xf) as usize;
    let code = ((hash[offset] & 0x7f) as u32) << 24
        | ((hash[offset + 1] & 0xff) as u32) << 16
        | ((hash[offset + 2] & 0xff) as u32) << 8
        | (hash[offset + 3] & 0xff) as u32;
    
    Ok(format!("{:06}", code % 1000000))
}

pub fn verify_otp_code(secret: &str, code: &str, timestamp: u64) -> Result<bool> {
    let generated_code = generate_otp_code(secret, timestamp)?;
    Ok(generated_code == code)
}

pub fn create_secure_random_password(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";
    let mut rng = rand::thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn validate_password_strength(password: &str) -> PasswordStrength {
    let mut score = 0;
    let mut feedback = Vec::new();
    
    if password.len() >= 8 {
        score += 1;
    } else {
        feedback.push("Password should be at least 8 characters long");
    }
    
    if password.chars().any(|c| c.is_uppercase()) {
        score += 1;
    } else {
        feedback.push("Password should contain uppercase letters");
    }
    
    if password.chars().any(|c| c.is_lowercase()) {
        score += 1;
    } else {
        feedback.push("Password should contain lowercase letters");
    }
    
    if password.chars().any(|c| c.is_numeric()) {
        score += 1;
    } else {
        feedback.push("Password should contain numbers");
    }
    
    if password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
        score += 1;
    } else {
        feedback.push("Password should contain special characters");
    }
    
    let strength = match score {
        0..=2 => "Weak",
        3..=4 => "Medium",
        5 => "Strong",
        _ => "Unknown",
    };
    
    PasswordStrength {
        score,
        strength: strength.to_string(),
        feedback,
    }
}

#[derive(Debug, Clone)]
pub struct PasswordStrength {
    pub score: u8,
    pub strength: String,
    pub feedback: Vec<String>,
}