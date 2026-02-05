use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::Aead; // AJOUTER CETTE LIGNE
use aes_gcm::KeyInit; // AJOUTER CETTE LIGNE
use argon2::Argon2;
use rand::rngs::OsRng;
use aes_gcm::AeadCore;
use std::fmt;

pub struct CryptoManager {
    cipher: Aes256Gcm,
}

#[derive(Debug)]
pub enum CryptoError {
    InvalidCiphertext,
    DecryptionFailed,
    InvalidUtf8,
}
impl std::error::Error for CryptoError {}


impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::InvalidCiphertext => write!(f, "invalid ciphertext"),
            CryptoError::DecryptionFailed => write!(f, "decryption failed"),
            CryptoError::InvalidUtf8 => write!(f, "invalid utf-8"),
        }
    }
}

impl CryptoManager {
    pub fn new(master_password: &str) -> Result<Self, CryptoError> {

        let salt = b"example salt";
        let mut key = [0u8; 32];
        Argon2::default().hash_password_into(master_password.as_bytes(), salt, &mut key).expect("Key derivation failure");
        let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| CryptoError::InvalidCiphertext)?; 
        Ok(CryptoManager { cipher })
    }

    pub fn encrypt(&self, data: &str) -> Result<Vec<u8>, CryptoError> {

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self.cipher.encrypt(&nonce, data.as_ref()).map_err(|_| CryptoError::DecryptionFailed)?;
        let mut res: Vec<u8> = nonce.to_vec();
        res.extend_from_slice(&ciphertext);
        Ok(res)
    }

    pub fn decrypt(&self, encrypted: Vec<u8>) -> Result<String, CryptoError> {

        let data = encrypted.split_at(12);
        let nonce = aes_gcm::Nonce::from_slice(data.0);
        let ciphertext = data.1;
        
        let plaintext = self.cipher.decrypt(nonce, ciphertext.as_ref()).map_err(|_| CryptoError::DecryptionFailed)?;
        let res = match str::from_utf8(&plaintext) {
            Ok(string) => string,
            Err(e)  => return Err(CryptoError::InvalidUtf8)
        }; 
        Ok(res.to_string())
    }
}