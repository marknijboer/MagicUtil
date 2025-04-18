use regex::Regex;
use serde_json::json;
use simple_error::SimpleError;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Cursor, Read, Write};

use crate::utils::print_error;

lazy_static! {
    pub static ref ENV_REG: Regex = Regex::new(r"^\$\{([a-zA-Z0-9_]+?)\}$").unwrap();
}

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub struct PropertiesMut {
    source: Box<dyn Read>,
    mutations: serde_json::Map<String, serde_json::Value>,
}

impl PropertiesMut {
    pub fn load(config_content: &str) -> Self {
        Self {
            source: Box::new(Cursor::new(config_content.to_string())),
            mutations: serde_json::Map::new(),
        }
    }

    /// Creates an instance of the PropertiesMut with the given file. That file
    /// should exist.
    pub fn open(path: &str) -> Result<Self, SimpleError> {
        let file = File::open(path).map_err(|e| {
            let message = format!("Could not load properties file: {e}");
            SimpleError::new(message)
        })?;

        Ok(Self {
            source: Box::new(file),
            mutations: serde_json::Map::new(),
        })
    }

    /// Adds a mutation with a new value for the current PropertiesMut
    pub fn set(&mut self, key: &str, value: &str) {
        let value_opt;

        // Check if the value is of format '${ENV_VARIABLE}'. If so, we will try
        // to insert the values from the environment variable.
        let matches_opt = ENV_REG.captures(value);
        if let Some(matches) = matches_opt {
            let env_key = &matches[1];
            if let Ok(env_val) = std::env::var(env_key) {
                value_opt = Some(env_val);
            } else {
                let error_message = format!("Value for key '{}' could not be inserted from environment variables because environment variable '{}' could not be resolved. An empty string will be used as value.", key, env_key);
                print_error(error_message);
                value_opt = Some(String::new());
            }
        } else {
            value_opt = Some(String::from(value));
        }
        self.mutations.insert(String::from(key), json!(value_opt));
    }

    /// Adds a mutations that will remove a certain key from the PropertiesMut
    pub fn remove(&mut self, key: &str) {
        self.mutations.insert(String::from(key), json!(null));
    }

    /// Returns a hashmap containing all values from the config.properties file.
    pub fn get_hashmap_content(&mut self) -> Result<HashMap<String, String>, SimpleError> {
        let mut content_map = HashMap::new();

        let lines: Vec<String> = BufReader::new(&mut self.source)
            .lines()
            .collect::<Result<_, _>>()
            .map_err(|e| SimpleError::new(format!("Could not load properties file: {}", e)))?;

        // Now process each line with an immutable borrow of self.
        for line in lines {
            if let Some((key, value)) = self.get_key_values(&line) {
                content_map.insert(key, value);
            }
        }

        Ok(content_map)
    }

    /// Returns the content of the config.properties file with the mutations
    /// applied as a string.
    pub fn get_mutated_content(&mut self) -> Result<String, SimpleError> {
        let mut property_buffer = String::new();
        let mut mutations_clone = self.mutations.clone();

        // Read all lines first, ending the mutable borrow on self.source.
        let lines: Vec<String> = BufReader::new(&mut self.source)
            .lines()
            .collect::<Result<_, _>>()
            .map_err(|e| SimpleError::new(format!("Could not load properties file: {e}")))?;

        // Now process each line after self.source is no longer borrowed.
        for line in lines {
            let key_opt = self.get_key(&line);
            if let Some(key) = key_opt {
                if mutations_clone.contains_key(&key) {
                    // If a mutated value exists, replace the current property.
                    let mutated_value = mutations_clone.get(&key).unwrap().clone();
                    if mutated_value.is_string() {
                        let value = mutated_value.as_str().unwrap();
                        let new_value = format!("{key} = {value}{LINE_ENDING}");
                        property_buffer.push_str(&new_value);
                    }
                    mutations_clone.remove(&key);
                    continue;
                }
            }

            // Non-matching lines are added back unchanged.
            let old_value = format!("{line}{LINE_ENDING}");
            property_buffer.push_str(&old_value);
        }

        // Append properties not found in the original file.
        for (key, mutated_value) in mutations_clone {
            if mutated_value.is_string() {
                let value = mutated_value.as_str().unwrap();
                let new_value = format!("{key} = {value}{LINE_ENDING}");
                property_buffer.push_str(&new_value);
            }
        }

        Ok(property_buffer)
    }

    /// Writes the content of the config.properties file with the mutations
    /// applied to the original config.properties file.
    pub fn write(&mut self, filename: &str) -> Result<(), SimpleError> {
        let content = self.get_mutated_content()?;

        let file_res = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename);

        if let Err(e) = file_res {
            let message = format!("Could not write properties file: {e}");
            return Err(SimpleError::new(message));
        }

        let mut file = file_res.unwrap();
        if let Err(e) = file.write_all(content.as_bytes()) {
            let message = format!("Could not write properties file: {e}");
            return Err(SimpleError::new(message));
        }

        Ok(())
    }

    /// A private method that tries to find the properties key from the given
    /// line. Returns an Option containing the key or None if no key was found.
    fn get_key(&self, line: &str) -> Option<String> {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with("#") || trimmed_line.starts_with("!")
        {
            return None;
        }

        if !trimmed_line.contains("=") {
            return None;
        }

        let splitted_line = trimmed_line.split("=").collect::<Vec<&str>>();
        let splitted_key = splitted_line[0].trim();
        if splitted_key.is_empty() {
            return None;
        }

        Some(String::from(splitted_key))
    }

    /// A private method that tries to find the properties key/values from the
    /// given line. Returns an Option containing the key or None if no key was
    /// found.
    fn get_key_values(&self, line: &str) -> Option<(String, String)> {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with("#") || trimmed_line.starts_with("!")
        {
            return None;
        }

        if !trimmed_line.contains("=") {
            return None;
        }

        let splitted_line = trimmed_line.split("=").collect::<Vec<&str>>();
        let splitted_key = splitted_line[0].trim();
        if splitted_key.is_empty() {
            return None;
        }

        let key = String::from(splitted_key);
        let value = splitted_line[1..].join("=").trim().to_owned();

        Some((key, value))
    }
}
