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


//test generer par ia 
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    
    #[test]
    fn test_parse_simple_config() {
        let config_str = r#"
[database]
host=localhost
port=5432

[server]
address=0.0.0.0
port=8080
"#;
        let config = Config::parse(config_str).unwrap();
        
        assert_eq!(config.sections.len(), 2);
        assert!(config.sections.contains_key("database"));
        assert!(config.sections.contains_key("server"));
        
        let db_section = &config.sections["database"];
        assert_eq!(db_section.properties.get("host"), Some(&"localhost".to_string()));
        assert_eq!(db_section.properties.get("port"), Some(&"5432".to_string()));
    }

    #[test]
    fn test_parse_with_comments() {
        let config_str = r#"
# Ceci est un commentaire
[app]
name=MyApp
# Encore un commentaire
version=1.0
"#;
        let config = Config::parse(config_str).unwrap();
        
        let app_section = &config.sections["app"];
        assert_eq!(app_section.properties.len(), 2);
        assert_eq!(app_section.properties.get("name"), Some(&"MyApp".to_string()));
    }

    #[test]
    fn test_duplicate_key_error() {
        let config_str = r#"
[test]
key=value1
key=value2
"#;
        let result = Config::parse(config_str);
        assert!(result.is_err());
        
        if let Err(ConfigError::DuplicateKey(key)) = result {
            assert_eq!(key, "key");
        } else {
            panic!("Expected DuplicateKey error");
        }
    }

    #[test]
    fn test_property_outside_section_error() {
        let config_str = r#"
key=value
[section]
key2=value2
"#;
        let result = Config::parse(config_str);
        assert!(result.is_err());
        
        if let Err(ConfigError::InvalidFormat(msg)) = result {
            assert!(msg.contains("Propriété en dehors de toute section"));
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    #[test]
    fn test_invalid_line_error() {
        let config_str = r#"
[section]
invalid line without equals
"#;
        let result = Config::parse(config_str);
        assert!(result.is_err());
        
        if let Err(ConfigError::InvalidFormat(msg)) = result {
            assert!(msg.contains("Ligne non reconnue"));
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    
    #[test]
    fn test_get_value_exists() {
        let config_str = r#"
[database]
host=localhost
port=5432
"#;
        let config = Config::parse(config_str).unwrap();
        
        assert_eq!(config.get_value("database", "host"), Some(&"localhost".to_string()));
        assert_eq!(config.get_value("database", "port"), Some(&"5432".to_string()));
    }

    #[test]
    fn test_get_value_section_not_found() {
        let config_str = r#"
[database]
host=localhost
"#;
        let config = Config::parse(config_str).unwrap();
        
        assert_eq!(config.get_value("server", "host"), None);
    }

    #[test]
    fn test_get_value_key_not_found() {
        let config_str = r#"
[database]
host=localhost
"#;
        let config = Config::parse(config_str).unwrap();
        
        assert_eq!(config.get_value("database", "port"), None);
    }

    #[test]
    fn test_get_value_multiple_sections() {
        let config_str = r#"
[section1]
key1=value1

[section2]
key2=value2
"#;
        let config = Config::parse(config_str).unwrap();
        
        assert_eq!(config.get_value("section1", "key1"), Some(&"value1".to_string()));
        assert_eq!(config.get_value("section2", "key2"), Some(&"value2".to_string()));
        assert_eq!(config.get_value("section1", "key2"), None);
    }

    
    #[test]
    fn test_from_file_success() {
        let temp_path = "test_config_temp.ini";
        let config_content = r#"
[app]
name=TestApp
version=2.0

[database]
host=127.0.0.1
"#;
        fs::write(temp_path, config_content).unwrap();
        
        let config = Config::from_file(temp_path).unwrap();
        
        assert_eq!(config.sections.len(), 2);
        assert_eq!(config.get_value("app", "name"), Some(&"TestApp".to_string()));
        assert_eq!(config.get_value("database", "host"), Some(&"127.0.0.1".to_string()));
        
        fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn test_from_file_not_found() {
        let result = Config::from_file("fichier_inexistant.ini");
        assert!(result.is_err());
        
        if let Err(ConfigError::IoError(_)) = result {
        } else {
            panic!("Expected IoError");
        }
    }

    #[test]
    fn test_from_file_invalid_content() {
        let temp_path = "test_invalid_config.ini";
        let invalid_content = r#"
[section]
ligne invalide sans égal
"#;
        fs::write(temp_path, invalid_content).unwrap();
        
        let result = Config::from_file(temp_path);
        assert!(result.is_err());
        
        if let Err(ConfigError::InvalidFormat(_)) = result {
        } else {
            panic!("Expected InvalidFormat error");
        }
        
        fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn test_from_file_with_comments_and_empty_lines() {
        let temp_path = "test_with_comments.ini";
        let content = r#"
# Commentaire
[config]
key=value

# Autre commentaire

[other]
data=info
"#;
        fs::write(temp_path, content).unwrap();
        
        let config = Config::from_file(temp_path).unwrap();
        assert_eq!(config.sections.len(), 2);
        assert_eq!(config.get_value("config", "key"), Some(&"value".to_string()));
        
        fs::remove_file(temp_path).unwrap();
    }

    
    #[test]
    fn test_full_workflow() {
        let temp_path = "test_workflow.ini";
        let content = r#"
[server]
host=0.0.0.0
port=3000

[database]
url=postgres://localhost/mydb
max_connections=100
"#;
        fs::write(temp_path, content).unwrap();
        
        let config = Config::from_file(temp_path).unwrap();
        
        assert_eq!(config.get_value("server", "host"), Some(&"0.0.0.0".to_string()));
        assert_eq!(config.get_value("server", "port"), Some(&"3000".to_string()));
        assert_eq!(config.get_value("database", "url"), Some(&"postgres://localhost/mydb".to_string()));
        assert_eq!(config.get_value("database", "max_connections"), Some(&"100".to_string()));
        
        assert_eq!(config.get_value("server", "timeout"), None);
        assert_eq!(config.get_value("cache", "enabled"), None);
        
        fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn test_display_errors() {
        let err1 = ConfigError::InvalidFormat("test".to_string());
        assert_eq!(format!("{}", err1), "Format invalide : test");
        
        let err2 = ConfigError::DuplicateKey("mykey".to_string());
        assert_eq!(format!("{}", err2), "Clé dupliquée : mykey");
        
        let err3 = ConfigError::MissingSection("mysection".to_string());
        assert_eq!(format!("{}", err3), "Section manquante : mysection");
    }
}
