use std::usize;
use std::{fs, path::Path};
use rand::Rng;
use crate::crypto::{CryptoManager, CryptoError};
use crate::models::{PasswordStore, PasswordEntry};

pub struct Storage {
    pub crypto :CryptoManager,
}

#[derive(Debug)]
pub enum StorageError {
    IoError(String),
    CryptoError(String),
    SerdeError(String),
}
impl std::error::Error for StorageError {}


impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::IoError(msg) => write!(f, "I/O error: {}", msg),
            StorageError::CryptoError(msg) => write!(f, "Crypto error: {}", msg),
            StorageError::SerdeError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

const FILE_PATH: &str = ".pwdmgr_store.encrypted";


impl Storage {
    pub fn load(&self ) -> Result<PasswordStore, StorageError> {
        if !Path::new(FILE_PATH).exists() {
            return Ok(PasswordStore { entries: Vec::new() });
        }
        let encrypted_data = fs::read(FILE_PATH).map_err(|e| StorageError::IoError(e.to_string()))?;
        let decrypted_data = self.crypto.decrypt(encrypted_data).map_err(|e| StorageError::CryptoError(e.to_string()))?;
        let store: PasswordStore = serde_json::from_str(&decrypted_data).map_err(|e| StorageError::SerdeError(e.to_string()))?;
        Ok(store)
    }

    pub fn save(&self, store: &PasswordStore) -> Result<(), StorageError> {
        let json_data = serde_json::to_string(&store).map_err(|e| StorageError::SerdeError(e.to_string()))?;
        let encrypted_data = self.crypto.encrypt(&json_data).map_err(|e| StorageError::CryptoError(e.to_string()))?;
        fs::write(FILE_PATH, encrypted_data).map_err(|e| StorageError::IoError(e.to_string()))?;
        Ok(())
    }

    pub fn generate_password(&self, len: usize) -> String {
        let charset = b"AZERTYUIOPQSDFGHJKLMWXCVBNazertyuiopmlkjhgfdsqwxcvbn1234567890_!$()&-/:*%.;,?^+=}]@`|[{#~";
        let mut rng = rand::thread_rng();
        (0..len).map(|_| {
            let idx= rng.gen_range(0..charset.len());
            charset[idx] as char
        }).collect()
    }
}