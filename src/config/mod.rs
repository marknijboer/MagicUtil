mod config_util;
mod properties;
mod encrypted;

pub use config_util::get_config_properties_path;
pub use config_util::get_mi_home_dir;
pub use config_util::get_config_properties;

use clap::ArgMatches;
use simple_error::SimpleError;
use std::{collections::HashMap, path::PathBuf, process::exit};

use crate::utils::{print_as_json, print_as_lines, print_error};

use self::properties::PropertiesMut;

const LOG_PROPERTY: &str = "repeater.log.path";
const ENCRYPTION_KEY_PROPERTY: &str = "encrypt.manager.key.v1";

/// Returns the configuration values in the order in which the properties are
/// requested
pub fn handle_config_command(submatches: &ArgMatches) {
    match submatches.subcommand() {
        Some(("get", subsubmatches)) => get_config_values(subsubmatches),
        Some(("set", subsubmatches)) => set_config_value(subsubmatches),
        Some(("replace", subsubmatches)) => replace_config_value(subsubmatches),
        Some(("remove", subsubmatches)) => remove_config_value(subsubmatches),
        Some(("overlay", subsubmatches)) => overlay_config_values(subsubmatches),
        _ => {
            unreachable!("No valid subcommand found")
        }
    }
}

/// Returns one or more config property values
fn get_config_values(submatches: &ArgMatches) {
    let properties: Vec<&String> = submatches.get_many("PROPERTY").unwrap().collect();
    if properties.is_empty() {
        print_error("Expected one or more property keys");
        exit(1);
    }

    let properties_str: Vec<&str> = properties.into_iter().map(|p| {
        p.as_str()
    }).collect();

    let property_values_res = config_util::get_config_properties(&properties_str);
    if let Err(e) = property_values_res {
        print_error(e);
        exit(1);
    }

    let mut property_values = property_values_res.unwrap();
    if submatches.get_flag("decrypt") {
        let encryption_key_res = get_encryption_key();
        if let Err(e) = encryption_key_res {
            print_error(e);
        } else {
            decrypt_hashmap(&mut property_values, &encryption_key_res.unwrap());
        }
    }

    if submatches.get_flag("json") {
        print_as_json(property_values);
        return;
    }

    print_as_lines(property_values, &properties_str);
    return;
}

/// Reads the properties-file used as base and overlays the properties from the 
/// overlay properties-file. The resulting configuration file will be printed
/// to stdout.
fn overlay_config_values(submatches: &ArgMatches) {
    let base_config = submatches.get_one::<String>("BASE_CONFIG_INI").unwrap();
    let overlay_config = submatches.get_one::<String>("OVERLAY_CONFIG_INI").unwrap();

    // Load the base configuration file
    let base_config_properties_res = PropertiesMut::open(base_config);
    if base_config_properties_res.is_err() {
        print_error("Could not read the base properties-file.");
        exit(1);
    }
    let mut base_config_properties = base_config_properties_res.unwrap();

    // Load the overlay configuration file
    let overlay_config_properties_res = PropertiesMut::open(overlay_config);
    if overlay_config_properties_res.is_err() {
        print_error("Could not read the overlay properties-file.");
        exit(1);
    }

    // Read the overlay configuration file as hashmap and apply every key-value pair on the base configuration
    let config_changes = overlay_config_properties_res.unwrap().get_hashmap_content().unwrap();
    config_changes.iter().for_each(|(config_key, config_value)| {
        base_config_properties.set(config_key, config_value);
    });

    // Construct the final configuration file and print it to stdout.
    let content = base_config_properties.get_mutated_content().unwrap();
    println!("{}", content);
}

/// Edits one config property value by doing a search and replace on it.
fn replace_config_value(submatches: &ArgMatches) {
    let key = submatches.get_one::<String>("KEY").unwrap();
    let search = submatches.get_one::<String>("SEARCH").unwrap();
    let replace = submatches.get_one::<String>("REPLACE").unwrap();

    if key.is_empty() || search.is_empty() || replace.is_empty() {
        print_error("Expected a key, search and replace argument");
        exit(1);
    }

    // Find the current value of this key
    let current_property_values_res = config_util::get_config_properties(&[key]);
    if let Err(e) = current_property_values_res {
        print_error(e);
        exit(1);
    }
    let current_property_values = current_property_values_res.unwrap();
    let current_value_opt = current_property_values.get(key).unwrap();
    if current_value_opt.is_none() {
        let error_message = format!("key {key} is currently not set. Cannot execute replace on this key");
        print_error(error_message);
        exit(1);
    }

    let current_value = current_value_opt.clone().unwrap();
    let new_value = current_value.replace(search, replace);

    let property_res = get_property_mut();
    if let Err(e) = property_res {
        print_error(e);
        exit(1);
    }

    let mut property = property_res.unwrap();
    property.set(key, &new_value);
    let property_write_res = property.write();
    if let Err(e) = property_write_res {
        print_error(e);
        exit(1);
    }
}

/// Sets one config property value
fn set_config_value(submatches: &ArgMatches) {
    let key = submatches.get_one::<String>("KEY").unwrap();
    let mut value = submatches.get_one::<String>("VALUE").unwrap().to_owned();

    if key.is_empty() || value.is_empty() {
        print_error("Expected one key and one value");
        exit(1);
    }

    let property_res = get_property_mut();
    if let Err(e) = property_res {
        print_error(e);
        exit(1);
    }

    if submatches.get_flag("encrypt") {
        let encryption_key_res = get_encryption_key();
        if let Err(e) = encryption_key_res {
            print_error(e);
            exit(1);
        }
        let encryption_res = encrypted::aes_128_ecb_encrypt(&encryption_key_res.unwrap(), &value);
        if let Err(e) = encryption_res {
            print_error(e);
            exit(1);
        }

        value = encryption_res.unwrap();
    }

    let mut property = property_res.unwrap();
    property.set(key, &value);
    let property_write_res = property.write();
    if let Err(e) = property_write_res {
        print_error(e);
        exit(1);
    }
}

/// Removes a config property value
fn remove_config_value(submatches: &ArgMatches) {
    let key = submatches.get_one::<String>("KEY").unwrap();

    if key.is_empty() {
        print_error("Expected a key to remove");
        exit(1);
    }

    let property_res = get_property_mut();
    if let Err(e) = property_res {
        print_error(e);
        exit(1);
    }

    let mut property = property_res.unwrap();
    property.remove(key);
    let property_write_res = property.write();
    if let Err(e) = property_write_res {
        print_error(e);
        exit(1);
    }
}

/// Returns the log directory path
pub fn get_log_directory() -> PathBuf {
    let property_values_res = config_util::get_config_properties(&[LOG_PROPERTY]);
    if let Err(e) = property_values_res {
        print_error(e);
        exit(1);
    }

    let property_values = property_values_res.unwrap();
    let log_path_opt = property_values.get(LOG_PROPERTY).unwrap().clone();
    if log_path_opt.is_none() {
        print_error("The log path is not defined in the config.properties file");
        exit(1);
    }

    let log_path = log_path_opt.unwrap();
    PathBuf::from(log_path)
}

/// Returns the PropertiesMut from the MagicINFO's main config.properties file.
fn get_property_mut() -> Result<PropertiesMut, SimpleError>{
    let config_path = get_config_properties_path()?;
    let config_path_str = config_path.to_str().unwrap();
    let property_mut = properties::PropertiesMut::open(config_path_str)?;

    Ok(property_mut)
}

// Tries to decrypt all values in the hashmap. If a value fails, it will be skipped.
fn decrypt_hashmap(map: &mut HashMap<String, Option<String>>, encryption_key: &str) {
    let mut mutations: HashMap<String, String> = HashMap::new();
    for (key, value_opt) in map.iter() {
        if value_opt.is_some() {
            let value = value_opt.clone().unwrap();
            let decryption_res = encrypted::aes_128_ecb_decrypt(encryption_key, &value);
            if let Ok(decrypted_string) = decryption_res {
                mutations.insert(key.clone(), decrypted_string);
            }
        }
    }

    for (mutation_key, mutation_value) in mutations {
        map.insert(mutation_key, Some(mutation_value));
    }
}

// Searches for the encryption key in the config.properties file.
fn get_encryption_key() -> Result<String, SimpleError> {
    let config_list = config_util::get_config_properties(&[ENCRYPTION_KEY_PROPERTY])?;
    let encryption_key_opt = config_list.get(ENCRYPTION_KEY_PROPERTY).unwrap().clone();

    if encryption_key_opt.is_none() {
        return Err(SimpleError::new("The encryption key could not be found in the config.properties."));
    }

    Ok(encryption_key_opt.unwrap())
}