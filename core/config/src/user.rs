use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserConfig {
    pub author: Option<String>,
    pub group: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "description")]
    pub desc: Option<String>,
    pub username: Option<String>,
}

impl UserConfig {
    pub fn load() -> Self {
        let path = config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(cfg) = serde_yaml::from_str(&content) {
                    return cfg;
                }
            }
        }
        UserConfig::default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let content = serde_yaml::to_string(self).map_err(|e| e.to_string())?;
        fs::write(path, content).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn set(&mut self, key: &str, value: String) -> Result<(), String> {
        match key {
            "author" => self.author = Some(value),
            "group" => self.group = Some(value),
            "version" => self.version = Some(value),
            "desc" | "description" => self.desc = Some(value),
            "username" => self.username = Some(value),
            _ => return Err(format!("Unknown config key: {}", key)),
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "author" => self.author.clone(),
            "group" => self.group.clone(),
            "version" => self.version.clone(),
            "desc" | "description" => self.desc.clone(),
            "username" => self.username.clone(),
            _ => None,
        }
    }
}

pub fn config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".hymod");
    path.push("config.yaml");
    path
}
