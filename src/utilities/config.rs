use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ia: Ia,
}

#[derive(Debug, Deserialize)]
pub struct Ia {
    pub api_key: String,
    pub api_url: String,
    pub pront_ia: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(not(windows))]
        {
            let config_path = dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("svcheck")
                .join("config.toml");
            println!("{}", &config_path.to_str().unwrap());

            if !config_path.exists() {
                return Err("❌ Archivo de configuración no encontrado.".into());
            }

            let contents = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&contents)?;
            Ok(config)
        }
        #[cfg(windows)]
        {
            let config_path = std::env::current_exe()?.parent().unwrap().to_path_buf().join("config.toml");            
            println!("{}", &config_path.to_str().unwrap());

            if !config_path.exists() {
                return Err("❌ Archivo de configuración no encontrado.".into());
            }

            let contents = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&contents)?;
            Ok(config)
        }

    }
}
