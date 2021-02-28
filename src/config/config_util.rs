use super::properties::PropertiesMut;
use simple_error::SimpleError;
use std::{collections::HashMap, process::exit};
use std::env;
use std::path::PathBuf;

type ConfigValueMap = HashMap<String, Option<String>>;

/// Returns the full path to the config.properties file.
pub fn get_config_properties_path() -> Result<PathBuf, SimpleError> {
    let mut config_properties_path = get_mi_home_dir();
    config_properties_path.push("conf");
    config_properties_path.push("config.properties");

    if !config_properties_path.as_path().exists() {
        let error_message = format!("Could not find the config.properties at: {}", config_properties_path.display());
        return Err(SimpleError::new(error_message));
    }

    Ok(config_properties_path)
}

/// Returns the MagicINFO home directory.
pub fn get_mi_home_dir() -> PathBuf {
    let home_path_res = env::var("MAGICINFO_PREMIUM_HOME");
    if let Err(e) = home_path_res {
        let error_message = format!("Could not find environment variable `MAGICINFO_PREMIUM_HOME`: {}", e.to_string());
        eprintln!("{}", error_message);
        exit(1);
    }

    let home_path = home_path_res.unwrap();
    let mut config_dir = PathBuf::new();
    config_dir.push(home_path);

    config_dir
}

/// Searches in the config.properties file for the requested properties.
pub fn get_config_properties(properties: &[&str]) -> Result<ConfigValueMap, SimpleError> {
    let config_properties_path = get_config_properties_path()?;
    let config_properties_res = PropertiesMut::open(config_properties_path.to_str().unwrap());
    if config_properties_res.is_err() {
        return Err(SimpleError::new("Could not read the config.properties file."));
    }

    let config_properties = config_properties_res.unwrap();
    let configuration = config_properties.get_hashmap_content().unwrap();

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
