use std::fmt;
use std::fs;
use std::io;

pub struct FileManager;

#[derive(Debug)]
pub enum FileManagerError {
    FileNotFound,
    PermissionDenied,
    InvalidPath,
    IoError(String),
}

impl fmt::Display for FileManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileManagerError::FileNotFound => write!(f, "File not found."),
            FileManagerError::PermissionDenied => write!(f, "Permission denied."),
            FileManagerError::InvalidPath => write!(f, "Invalid file path"),
            FileManagerError::IoError(msg) => write!(f, "Erreur : {}", msg),
        }
    }
}

impl std::error::Error for FileManagerError {}

impl FileManager {
    pub fn list_files(path: &str) -> Result<Vec<String>, FileManagerError> {
        let entries = fs::read_dir(path).map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => FileManagerError::FileNotFound,
            io::ErrorKind::PermissionDenied => FileManagerError::PermissionDenied,
            _ => FileManagerError::IoError(e.to_string()),
        })?;

        let mut file_names = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|e| FileManagerError::IoError(e.to_string()))?;
            let file_type = entry.file_type().map_err(|e| FileManagerError::IoError(e.to_string()))?;

            if file_type.is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    file_names.push(name.to_string());
                }
            }
        }
        Ok(file_names)
    }

    pub fn read_file(path: &str) -> Result<String, FileManagerError> {
        fs::read_to_string(path).map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => FileManagerError::FileNotFound,
            io::ErrorKind::PermissionDenied => FileManagerError::PermissionDenied,
            io::ErrorKind::InvalidInput => FileManagerError::InvalidPath,
            _ => FileManagerError::IoError(e.to_string()),
        })
    }

    pub fn write_file(path: &str, msg: &str) -> Result<(), FileManagerError> {
        fs::write(path, msg).map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => FileManagerError::FileNotFound,
            io::ErrorKind::PermissionDenied => FileManagerError::PermissionDenied,
            io::ErrorKind::InvalidInput => FileManagerError::InvalidPath,
            _ => FileManagerError::IoError(e.to_string()),
        })
    }

    pub fn copy_file(src: &str, dst: &str) -> Result<(), FileManagerError> {
        fs::copy(src, dst).map(|_| ()).map_err(|e| match e.kind() {
                io::ErrorKind::NotFound => FileManagerError::FileNotFound,
                io::ErrorKind::PermissionDenied => FileManagerError::PermissionDenied,
                io::ErrorKind::InvalidInput => FileManagerError::InvalidPath,
                _ => FileManagerError::IoError(e.to_string()),
            })
    }
}