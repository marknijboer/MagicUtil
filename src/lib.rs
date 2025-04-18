#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

mod bcrypt;
mod cli;
mod config;
mod info;
mod open;
mod service;
mod system;
mod utils;

use config::properties::PropertiesMut;

use libc::c_char;
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
};

const CONVERSION_ERROR: &str = "cannot convert output to cstring";

/// Returns all the properties of the config.properties content in JSON format
#[no_mangle]
pub extern "C" fn get_all_properties(c_config_content: *const c_char) -> *mut c_char {
    let config_content_cstr = unsafe { CStr::from_ptr(c_config_content) };
    let config_content = config_content_cstr.to_str().unwrap();

    let mut properties_mut = PropertiesMut::load(config_content);
    let properties = properties_mut.get_hashmap_content().unwrap_or_default();

    let output = serde_json::ser::to_string(&properties).unwrap();
    let cstr = CString::new(output).expect(CONVERSION_ERROR);
    cstr.into_raw()
}

/// Returns an updated version of the config file with the properties set.
#[no_mangle]
pub extern "C" fn set_properties(
    c_config_content: *const c_char,
    c_changed_properties_json: *const c_char,
) -> *mut c_char {
    let config_content_cstr = unsafe { CStr::from_ptr(c_config_content) };
    let config_content = config_content_cstr.to_str().unwrap();

    let changed_properties_json_cstr = unsafe { CStr::from_ptr(c_changed_properties_json) };
    let changed_properties_json_str = changed_properties_json_cstr.to_str().unwrap();

    // Deserialize the JSON string into a serde_json::Map<String, String>
    let changed_properties = match serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
        changed_properties_json_str,
    ) {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Failed to deserialize JSON: {:?}", e);
            // Return a null pointer on error
            return std::ptr::null_mut();
        }
    };

    let mut properties_mut = PropertiesMut::load(config_content);
    for (key, value) in changed_properties {
        properties_mut.set(&key, value.as_str().unwrap());
    }
    let config = properties_mut.get_mutated_content().unwrap();
    let cstr = CString::new(config).expect(CONVERSION_ERROR);
    cstr.into_raw()
}

/// Returns all the properties from the second config file that were not present or
/// differently defined in the template config file.
#[no_mangle]
pub extern "C" fn get_properties_diff(
    c_template_content: *const c_char,
    c_config_content: *const c_char,
) -> *mut c_char {
    let template_content_cstr = unsafe { CStr::from_ptr(c_template_content) };
    let template_content = template_content_cstr.to_str().unwrap();
    let mut template_properties_mut = PropertiesMut::load(template_content);
    let template_properties = template_properties_mut
        .get_hashmap_content()
        .unwrap_or_default();

    let config_content_cstr = unsafe { CStr::from_ptr(c_config_content) };
    let config_content = config_content_cstr.to_str().unwrap();
    let mut config_properties_mut = PropertiesMut::load(config_content);
    let config_properties = config_properties_mut
        .get_hashmap_content()
        .unwrap_or_default();

    let changes_compared_to_base: HashMap<&str, &str> = config_properties
        .iter()
        .filter_map(|(config_key, config_value)| {
            let base_config_value = template_properties.get(config_key.as_str());
            if base_config_value.is_none() {
                return Some((config_key.as_str(), config_value.as_str()));
            }

            if base_config_value.unwrap() != config_value {
                return Some((config_key.as_str(), config_value.as_str()));
            }

            None
        })
        .collect();

    let output = serde_json::ser::to_string(&changes_compared_to_base).unwrap();
    let cstr = CString::new(output).expect(CONVERSION_ERROR);
    cstr.into_raw()
}
