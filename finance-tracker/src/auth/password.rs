use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use password_hash::{SaltString, PasswordHash};
use rand_core::OsRng; // for secure random salt
use std::error::Error;
pub fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| Box::new(e.to_string())).unwrap(); // Convert error to string
    Ok(password_hash.to_string())
}

pub fn verify_password(stored_hash: &str, password: &str) -> Result<bool, Box<dyn Error>> {
    let parsed_hash = PasswordHash::new(stored_hash)
        .map_err(|e| Box::new(e.to_string())).unwrap(); // Convert error to string
    let argon2 = Argon2::default();

    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}