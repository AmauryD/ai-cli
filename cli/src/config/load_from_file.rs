use core::configuration::Configuration;
use std::path::PathBuf;
use serde::Deserialize;
use super::load_config_error::LoadConfigError;

#[derive(Deserialize)]
pub struct CliConfiguration {
    gpt_url: String,
    model: String,
}

pub trait LoadConfig {
    fn load_config(&self) -> Result<Configuration, LoadConfigError>;
}

pub struct LoadConfigFromFile;

impl LoadConfig for LoadConfigFromFile{
    fn load_config(&self) -> Result<Configuration, LoadConfigError> {
        let config_path = 
            homedir::get_my_home().map_err(|_| LoadConfigError::FailedToGetHome)?
                .map(|path| {
                    PathBuf::from(format!("{}/.gpt-cli.toml", path.to_string_lossy()))
        }).ok_or(LoadConfigError::FailedToGetHome)?;
    
        if !config_path.exists() {
            return Err(LoadConfigError::ConfigDoesNotExists);
        }
    
        let config_string = std::fs::read_to_string(config_path).map_err(|_| LoadConfigError::CannotRead)?;
        let config: CliConfiguration = toml::from_str(&config_string).map_err(|_| LoadConfigError::FailedToParse)?;
    
        Ok(Configuration {
            gpt_url: config.gpt_url,
            model: config.model,
        })
    }
}

