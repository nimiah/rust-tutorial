use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::config::AppConfig;
use crate::models::auth::Claims;

pub struct Tokenizer {
    secret: String,
}

impl Tokenizer {
    pub fn new() -> Self {
        let config = AppConfig::get();

        Self {
            secret: config.secret_key.clone(),
        }
    }

    pub fn generate(self, claims: Claims) -> Result<String, String> {
        let encoding_key = EncodingKey::from_secret(self.secret.as_ref());
        encode(&Header::default(), &claims, &encoding_key)
            .map_err(|e| format!("Generate token failed (err = {:?})", e))
    }

    pub fn verify(self, token: &str) -> Result<Claims, String> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_ref());
        let validation = Validation::default();

        decode::<Claims>(token, &decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| format!("Invalid token (err = {:?})", e))
    }
}
