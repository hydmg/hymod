use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub server: ServerBlock,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerBlock {
    pub name: String,
    pub kind: ServerKind,
    pub server_root: String,
    pub mods_dir: String,
    pub restart: RestartBlock,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<RemoteBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<UploadBlock>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServerKind {
    Local,
    Remote,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestartBlock {
    pub cmd: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteBlock {
    pub host: String,
    pub user: String,
    #[serde(default = "default_ssh_port")]
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_hosts_file: Option<String>,
}

fn default_ssh_port() -> u16 {
    22
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadBlock {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsync: Option<RsyncBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scp: Option<ScpBlock>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RsyncBlock {
    pub opts: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScpBlock {
    pub opts: String,
}

pub fn get_server_config_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    Ok(home.join(".hymod").join("servers.d"))
}

pub fn save_server_config(config: &ServerConfig) -> Result<(), String> {
    let dir = get_server_config_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }

    let filename = format!("{}.yaml", config.server.name);
    let path = dir.join(filename);

    let content = serde_yaml::to_string(config).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn load_server_config(name: &str) -> Result<ServerConfig, String> {
    let dir = get_server_config_dir()?;
    let path = dir.join(format!("{}.yaml", name));

    if !path.exists() {
        return Err(format!("Server configuration '{}' not found", name));
    }

    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: ServerConfig = serde_yaml::from_str(&content).map_err(|e| e.to_string())?;

    Ok(config)
}

pub fn remove_server_config(name: &str) -> Result<(), String> {
    let dir = get_server_config_dir()?;
    let path = dir.join(format!("{}.yaml", name));

    if !path.exists() {
        return Err(format!("Server configuration '{}' not found", name));
    }

    fs::remove_file(path).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn list_servers() -> Result<Vec<String>, String> {
    let dir = get_server_config_dir()?;
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut servers = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if stem != "default" {
                    // Exclude "default" file if it ends up here (it shouldn't as it has no extension in my code)
                    servers.push(stem.to_string());
                }
            }
        }
    }
    servers.sort();
    Ok(servers)
}

pub fn get_default_server() -> Result<Option<String>, String> {
    let dir = get_server_config_dir()?;
    let path = dir.join("default");
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    Ok(Some(content.trim().to_string()))
}

pub fn set_default_server(name: &str) -> Result<(), String> {
    let dir = get_server_config_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    let path = dir.join("default");
    fs::write(path, name).map_err(|e| e.to_string())?;
    Ok(())
}
