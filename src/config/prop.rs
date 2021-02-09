use java_properties::read;
use simple_error::SimpleError;
use std::{collections::HashMap, fs::File};
use std::io::BufReader;
use std::env;
use std::path::PathBuf;

type ConfigValueMap = HashMap<String, Option<String>>;

/// Returns the full path to the config.properties file.
pub fn get_config_properties_path() -> Result<PathBuf, SimpleError> {
    let home_path_res = env::var("MAGICINFO_PREMIUM_HOME");
    if let Err(e) = home_path_res {
        let error_message = format!("Could not find environment variable `MAGICINFO_PREMIUM_HOME`: {}", e.to_string());
        return Err(SimpleError::new(error_message));
    }

    let home_path = home_path_res.unwrap();
    let mut config_properties_path = PathBuf::new();

    config_properties_path.push(home_path);
    config_properties_path.push("conf");
    config_properties_path.push("config.properties");

    if !config_properties_path.as_path().exists() {
        let error_message = format!("Could not find the config.properties at: {}", config_properties_path.display());
        return Err(SimpleError::new(error_message));
    }

    Ok(config_properties_path)
}

/// Searches in the config.properties file for the requested properties.
pub fn get_config_properties(properties: &[&str]) -> Result<ConfigValueMap, SimpleError> {
    let config_properties_path = get_config_properties_path()?;
    let config_properties = File::open(config_properties_path).unwrap();
    let configuration_res = read(BufReader::new(config_properties));

    if configuration_res.is_err() {
        return Err(SimpleError::new("Could not read the config.properties file."));
    }

    let configuration = configuration_res.unwrap();
    let mut config_map = ConfigValueMap::new();

    for property in properties {
        let value = if let Some(val) = configuration.get(property.to_owned()) {
            Some(val.clone())
        } else {
            None
        };

        config_map.insert(String::from(property.to_owned()), value);
    }

    Ok(config_map)
}