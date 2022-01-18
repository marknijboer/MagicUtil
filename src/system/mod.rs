mod prop;

use std::{collections::HashMap};
use clap::ArgMatches;

use crate::utils::{print_as_json};

/// Handles all system related commands.
pub fn handle_system_command(submatches: &ArgMatches) {
    match submatches.subcommand() {
        Some(("hwunique", subsubmatches)) => {
            print_system_value("hwunique", subsubmatches.is_present("json"))
        },
        Some(("macaddress", subsubmatches)) => {
            print_system_value("macaddress", subsubmatches.is_present("json"))
        },
        Some(("boardid", subsubmatches)) => {
            print_system_value("boardid", subsubmatches.is_present("json"))
        },
        Some(("ipaddress", subsubmatches)) => {
            print_system_value("ipaddress", subsubmatches.is_present("json"))
        },
        _ => {
            unreachable!("No valid subcommand found");
        }
    }
}

// Prints a single system value in the desired form.
fn print_system_value(key: &str, json: bool) {
    let property_values = get_system_values(&[key]);
    if json {
        print_as_json(property_values);
        return;
    }

    let value = property_values.get(key).unwrap();
    println!("{}", value.to_owned().unwrap_or_default());
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