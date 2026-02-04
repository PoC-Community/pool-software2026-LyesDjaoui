use std::collections::HashMap;
use std::fmt;
use std::io;
use std::fs;

#[derive(Debug)]
pub struct ConfigSection {
    pub name: String,
    pub properties: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Config {
    pub sections: HashMap<String, ConfigSection>,
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidFormat(String),
    MissingSection(String),
    DuplicateKey(String),
    IoError(io::Error), 
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidFormat(msg) => write!(f, "Format invalide : {}", msg),
            ConfigError::MissingSection(sec) => write!(f, "Section manquante : {}", sec),
            ConfigError::DuplicateKey(key) => write!(f, "Clé dupliquée : {}", key),
            ConfigError::IoError(err) => write!(f, "Erreur : {}", err),
        }
    }
}

impl Config {
    pub fn parse(contenue : &str) -> Result<Config, ConfigError> {
        let mut sections = HashMap::new();
        let mut current_section: Option<ConfigSection> = None;

        for (line_num, line) in contenue.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue; 
            }

            if line.starts_with('[') && line.ends_with(']') {
                if let Some(section) = current_section.take() {
                    sections.insert(section.name.clone(), section);
                }
                let section_name = &line[1..line.len() - 1];
                current_section = Some(ConfigSection {
                    name: section_name.to_string(),
                    properties: HashMap::new(),
                });
            } else if let Some(eq_pos) = line.find('=') {
                if let Some(section) = current_section.as_mut() {
                    let key = line[..eq_pos].trim().to_string();
                    let value = line[eq_pos + 1..].trim().to_string();
                    if section.properties.contains_key(&key) {
                        return Err(ConfigError::DuplicateKey(key));
                    }
                    section.properties.insert(key, value);
                } else {
                    return Err(ConfigError::InvalidFormat(format!(
                        "Propriété en dehors de toute section à la ligne {}",
                        line_num + 1
                    )));
                }
            } else {
                return Err(ConfigError::InvalidFormat(format!(
                    "Ligne non reconnue à la ligne {}",
                    line_num + 1
                )));
            }
        }

        if let Some(section) = current_section {
            sections.insert(section.name.clone(), section);
        }
        Ok(Config { sections })
    }

    //One line cadeau
    pub fn from_file(path: &str) -> Result<Config, ConfigError> {Config::parse(&std::fs::read_to_string(path).map_err(ConfigError::IoError)?)}

    pub fn get_value(&self, section_name: &str, key: &str) -> Option<&String> {self.sections.get(section_name)?.properties.get(key)}


}


