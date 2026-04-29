use base64::{Engine as _, engine::general_purpose};
use rand::{Rng, thread_rng};
use sha2::{Digest, Sha256};

pub struct PasswordUtil;

impl PasswordUtil {
    pub fn hash_password(password: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
        // Generate a random salt
        let mut rng = thread_rng();
        let salt: [u8; 32] = rng.r#gen();
        //let salt_hex = hex::encode(salt); Using Hex to encode byte to string saved in DB
        let salt_base64 = general_purpose::STANDARD.encode(salt); // Using Base64 to encode saved in DB

        // Hash password with salt using SHA256
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hasher.update(&salt);
        let hash = hasher.finalize();
        //let hash_hex = hex::encode(hash);
        let hash_base64 = general_purpose::STANDARD.encode(hash);
        //Ok((hash_hex, salt_hex))
        Ok((hash_base64, salt_base64))
    }

    pub fn verify_password(
        password: &str,
        hash: &str,
        salt: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // Hash the provided password with the same salt
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        //hasher.update(hex::decode(salt)?);

        //let computed_hash = hex::encode(hasher.finalize());

        hasher.update(general_purpose::STANDARD.decode(salt)?);
        let verify_hash = general_purpose::STANDARD.encode(hasher.finalize());

        Ok(verify_hash == hash)
    }
}
