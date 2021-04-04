use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Write};

use simple_error::SimpleError;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub struct PropertiesMut {
    source: String,
    mutations: HashMap<String, Option<String>>,
}

impl PropertiesMut {

    /// Creates an instance of the PropertiesMut with the given file. That file
    /// should exist.
    pub fn open(path: &str) -> Result<Self, SimpleError> {
        File::open(path).map_err(|e| {
            let message = format!("Could not load properties file: {}", e);
            SimpleError::new(message)
        })?;

        Ok(Self {
            source: String::from(path),
            mutations: HashMap::new(),
        })
    }

    /// Adds a mutation with a new value for the current PropertiesMut
    pub fn set(&mut self, key: &str, value: &str) {
        let value_opt = Some(String::from(value));
        &self.mutations.insert(String::from(key), value_opt);
    }

    /// Adds a mutations that will remove a certain key from the PropertiesMut
    pub fn remove(&mut self, key: &str) {
        &self.mutations.insert(String::from(key), None);
    }

    /// Returns a hashmap containing all values from the config.properties file.
    pub fn get_hashmap_content(&self) -> Result<HashMap<String, String>, SimpleError> {
        let file = File::open(&self.source).map_err(|e| {
            let message = format!("Could not load properties file: {}", e);
            SimpleError::new(message)
        })?;

        let mut content_map = HashMap::new();

        for line_res in BufReader::new(file).lines() {
            let line = line_res.map_err(|e| {
                let message = format!("Could not load properties file: {}", e);
                SimpleError::new(message)
            })?;

            let key_opt = self.get_key_values(&line);
            if key_opt.is_none() {
                continue;
            }

            let (key, value) = key_opt.unwrap();
            content_map.insert(key, value);
        }

        Ok(content_map)
    }

    /// Returns the content of the config.properties file with the mutations
    /// applied as a string.
    pub fn get_mutated_content(&self) -> Result<String, SimpleError> {
        let mut property_buffer = String::new();
        let mut mutations_clone = self.mutations.clone();

        let file = File::open(&self.source).map_err(|e| {
            let message = format!("Could not load properties file: {}", e);
            SimpleError::new(message)
        })?;

        // Read the file line by line and try to interpret it as a properties
        // key-value.
        for line_res in BufReader::new(file).lines() {
            let line = line_res.map_err(|e| {
                let message = format!("Could not load properties file: {}", e);
                SimpleError::new(message)
            })?;

            let key_opt = self.get_key(&line);
            if let Some(key) = key_opt {
                if mutations_clone.contains_key(&key) {

                    // `mutated_value` will be an option. If the key was removed
                    // with the `remove` method, it will be None. Else it will
                    // contain a value that should be the new value of this key.
                    let mutated_value = mutations_clone.get(&key).unwrap().clone();
                    if mutated_value.is_some() {
                        let new_value = format!("{} = {}{}", key, mutated_value.unwrap(), LINE_ENDING);
                        property_buffer.push_str(&new_value);
                    }

                    mutations_clone.remove(&key);
                    continue;
                }
            }

            // All non-matching lines will be placed back in the file without
            // being changed.
            let old_value = format!("{}{}", &line, LINE_ENDING);
            property_buffer.push_str(&old_value);
        }

        // All properties that were not resolved by the iterating the values of
        // the properties file will be added on the bottom of the file.
        for extra_properties in mutations_clone {
            let mutated_value = extra_properties.1;
            if mutated_value.is_some() {
                let new_value = format!("{} = {}{}", extra_properties.0, mutated_value.unwrap(), LINE_ENDING);
                property_buffer.push_str(&new_value);
            }

        }

        Ok(property_buffer)
    }

    /// Writes the content of the config.properties file with the mutations
    /// applied to the original config.properties file.
    pub fn write(&self) -> Result<(), SimpleError> {
        let content = self.get_mutated_content()?;

        let file_res = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.source);

        if let Err(e) = file_res {
            let message = format!("Could not write properties file: {}", e);
            return Err(SimpleError::new(message));
        }

        let mut file = file_res.unwrap();
        if let Err(e) = file.write_all(content.as_bytes()) {
            let message = format!("Could not write properties file: {}", e);
            return Err(SimpleError::new(message));
        }

        Ok(())
    }

    /// A private method that tries to find the properties key from the given
    /// line. Returns an Option containing the key or None if no key was found.
    fn get_key(&self, line: &str) -> Option<String> {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with("#") || trimmed_line.starts_with("!") {
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
        if trimmed_line.is_empty() || trimmed_line.starts_with("#") || trimmed_line.starts_with("!") {
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
        let value = splitted_line[1..]
            .join("=")
            .trim()
            .to_owned();

        Some((key, value))
    }
}