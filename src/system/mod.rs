mod prop;

use std::{collections::HashMap, process::exit};
use clap::ArgMatches;

use crate::utils::{print_as_json, print_as_lines};

/// Handles all system related commands.
pub fn handle_system_command(submatches: &ArgMatches) {
    let properties: Vec<&str> = submatches.values_of("PROPERTY").unwrap().collect();
    if properties.is_empty() {
        eprintln!("Expected one or more property keys");
        exit(1);
    }

    let property_values = get_system_values(&properties);
    if submatches.is_present("json") {
        print_as_json(property_values);
        return;
    }

    print_as_lines(property_values, &properties);
    return;
}

/// Returns a hashmap containing values for the list of properties given as an
/// the argument.
pub fn get_system_values(properties: &[&str])  -> HashMap<String, Option<String>> {
    let mut value_list = HashMap::new();
    for property in properties {
        let value = get_system_value(property.to_owned());
        value_list.insert(String::from(property.to_owned()), value);
    }

    value_list
}

/// Returns a single system value for a given key
fn get_system_value(key: &str) -> Option<String> {
    match key {
        "boardid" => Some(prop::get_board_id()),
        "hwunique" => Some(prop::get_hwunique()),
        "macaddress" => Some(prop::get_mac_address()),
        "ipaddress" => Some(prop::get_ip_address()),
        _ => None
    }
}