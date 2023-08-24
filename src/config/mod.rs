use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{services::epik::Epik, traits::domain_control::DomainController};

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Config file doesn't exist in the provided (or default) path: {0}")]
    ConfigFileDoesNotExist(PathBuf),
    #[error("File exists but it could not be read to a string for parsing: {0}")]
    FileExistsButCannotBeReadToString(std::io::Error),
    #[error("Could not parse file to config; either invalid yaml or missing config: {0}")]
    FileFormatCouldNotBeParsed(serde_yaml::Error),
}

/// The configs of different providers
///
/// To add providers:
/// 1. Add a new struct with the provider's name
/// 2. Implement the DomainController trait for the struct
/// 3. Implement serde's Serialize and Deserialize for the struct
/// 4. Add the struct to the Config struct below in a Vec
/// 5. Add the parsing that struct to the function into_domain_controllers_map()
///
/// Would be nice to have a macro that does the last step automatically
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    // The list of Epik services to be used
    pub epik_configs: Vec<Epik>,
}

impl Config {
    pub fn from_file_or_default<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
        if !path.as_ref().exists() {
            return Err(ConfigError::ConfigFileDoesNotExist(
                path.as_ref().to_path_buf(),
            ));
        }

        let config_file_data = std::fs::read_to_string(path)
            .map_err(ConfigError::FileExistsButCannotBeReadToString)?;

        let config: Config = serde_yaml::from_str(&config_file_data)
            .map_err(ConfigError::FileFormatCouldNotBeParsed)?;

        Ok(config)
    }

    pub fn into_domain_controllers(
        self,
    ) -> std::collections::BTreeMap<String, Box<dyn DomainController>> {
        let provided_size = self.epik_configs.len();

        let result = self
            .epik_configs
            .into_iter()
            .map(|v| {
                (
                    v.domain_name().clone(),
                    Box::new(v) as Box<dyn DomainController>,
                )
            })
            .collect::<BTreeMap<_, _>>();

        // Check for duplicate domain configurations
        if result.len() != provided_size {
            panic!("Duplicate domain names in config file");
        }

        result
    }
}
