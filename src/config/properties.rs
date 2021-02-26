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
    mutations: HashMap<String, String>,
}

impl PropertiesMut {
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

    pub fn set(&mut self, key: &str, value: &str) {
        &self.mutations.insert(String::from(key), String::from(value));
    }

    pub fn get_content(&self) -> Result<String, SimpleError> {
        let mut property_buffer = String::new();
        let mut mutations_clone = self.mutations.clone();

        let file = File::open(&self.source).map_err(|e| {
            let message = format!("Could not load properties file: {}", e);
            SimpleError::new(message)
        })?;

        for line_res in BufReader::new(file).lines() {
            let line = line_res.map_err(|e| {
                let message = format!("Could not load properties file: {}", e);
                SimpleError::new(message)
            })?;

            let key_opt = self.get_key(&line);
            if let Some(key) = key_opt {
                if mutations_clone.contains_key(&key) {
                    let new_value = format!("{} = {}{}", key, mutations_clone.get(&key).unwrap(), LINE_ENDING);
                    property_buffer.push_str(&new_value);
                    mutations_clone.remove(&key);
                    continue;
                }
            }

            let old_value = format!("{}{}", &line, LINE_ENDING);
            property_buffer.push_str(&old_value);
        }

        for extra_properties in mutations_clone {
            let new_value = format!("{} = {}{}", extra_properties.0, extra_properties.1, LINE_ENDING);
            property_buffer.push_str(&new_value);
        }

        Ok(property_buffer)
    }

    pub fn write(&self) -> Result<(), SimpleError> {
        let content = self.get_content()?;

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

    fn get_key(&self, line: &str) -> Option<String> {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with("#") {
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
}