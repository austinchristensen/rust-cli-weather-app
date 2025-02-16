use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub default_city: String,
}

pub struct ConfigManager {
    pub config_file_path: String,
}

impl ConfigManager {
    pub fn new(config_file_path: &str) -> ConfigManager {
        ConfigManager {
            config_file_path: String::from(config_file_path),
        }
    }

    pub fn load_config(&self) -> Result<String, Error> {
        let mut file = match File::open(&self.config_file_path) {
            Ok(file) => file,
            Err(_) => return Err(Error::new(ErrorKind::NotFound, "Config file not found")),
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = serde_json::from_str(&contents)?;
        Ok(config.default_city)
    }

    pub fn save_config(&self, value: &str) -> Result<(), Error> {
        let config = Config {
            default_city: String::from(value),
        };

        let serialized_config = serde_json::to_string(&config)?;

        let mut file = File::create(&self.config_file_path)?;
        file.write_all(serialized_config.as_bytes())?;

        Ok(())
    }
}
