use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,  // User ID or Email
    exp: usize,   // Expiry Time
}

const SECRET_KEY: &[u8] = b"some_secret_key";

pub fn create_jwt(user_email: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600;  // Token expires in 1 hour

    let claims = Claims {
        sub: user_email.to_string(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(SECRET_KEY);
    encode(&header, &claims, &encoding_key).unwrap()
}

// pub fn validate_jwt(token: &str) -> bool {
//     let decoding_key = DecodingKey::from_secret(SECRET_KEY);
//     let mut validation = Validation::new(Algorithm::HS256);
//     validation.leeway = 0;
//     validation.validate_exp = true;
//     validation.validate_nbf = true;
//     validation.iss = None;
//     validation.aud = None;

//     match decode::<Claims>(&token, &decoding_key, &validation) {
//         Ok(_) => true,
//         Err(_) => false,
//     }
// }
