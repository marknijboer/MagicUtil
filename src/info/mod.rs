use std::{collections::HashMap, process::{exit}};

use clap::{ArgMatches};
use crate::{config::get_config_properties, utils::{print_as_lines_with_context, print_error}};
use crate::system::get_system_values;
use crate::utils::print_as_json;
use crate::service::get_service_status;

const MAGICINFO_INFO_ELEMS: &[&str] = &["wsrm.premiumVersion", "web_url", "CONTENTS_HOME"];
const DATABASE_INFO_ELEMS: &[&str] = &["wsrm.dbVendor", "wsrm.url", "wsrm.username", "wsrm.password"];
const SYSTEM_INFO_ELEMS: &[&str] = &["hwunique", "boardid", "macaddress", "ipaddress"];
const SERVICE_INFO_ELEMS: &[&str] = &["state", "serviceUser", "startMode"];

#[derive(Debug, Serialize)]
pub struct AllInfo {
    magicinfo: HashMap<String, Option<String>>,
    database: HashMap<String, Option<String>>,
    system: HashMap<String, Option<String>>,
    service: HashMap<String, Option<String>>,
}

/// Handles all info related commands.
pub fn handle_info_command(submatches: &ArgMatches) {
    match submatches.subcommand() {
        Some(("database", subsubmatches)) => print_config_based_properties(subsubmatches, DATABASE_INFO_ELEMS),
        Some(("magicinfo", subsubmatches)) => print_config_based_properties(subsubmatches, MAGICINFO_INFO_ELEMS),
        Some(("system", subsubmatches)) => {
            let system_properties = get_system_values(SYSTEM_INFO_ELEMS);
            if subsubmatches.is_present("json") {
                print_as_json(system_properties);
                return;
            }
    
            print_as_lines_with_context(system_properties, SYSTEM_INFO_ELEMS, None);
        },
        Some(("service", subsubmatches)) => {
            let service_properties = get_service_status();
            if subsubmatches.is_present("json") {
                print_as_json(service_properties);
                return
            }
            
            print_as_lines_with_context(service_properties, SERVICE_INFO_ELEMS, None);
        },
        Some(("all", subsubmatches)) => {
            let json_output = subsubmatches.is_present("json");
                if json_output {
                    print_all_info_as_json();
                    return;
                }
        
                print_all_info_as_lines();
        }, 
        _ => {
            unreachable!("No valid subcommand found")
        }
    }
}

/// Prints all information as plain text
fn print_all_info_as_lines() {
    println!("MagicINFO:");
    let magicinfo_props = get_config_values(MAGICINFO_INFO_ELEMS);
    print_as_lines_with_context(magicinfo_props, MAGICINFO_INFO_ELEMS, Some(20));

    println!("Database:");
    let database_props = get_config_values(DATABASE_INFO_ELEMS);
    print_as_lines_with_context(database_props, DATABASE_INFO_ELEMS, Some(20));

    println!("System:");
    let system_props = get_system_info();
    print_as_lines_with_context(system_props, SYSTEM_INFO_ELEMS, Some(20));

    println!("Service:");
    let service_props = get_service_status();
    print_as_lines_with_context(service_props, SERVICE_INFO_ELEMS, Some(20));
}

/// Prints all information as a json object
fn print_all_info_as_json() {
    let all_info = AllInfo{
        magicinfo: get_config_values(MAGICINFO_INFO_ELEMS),
        database: get_config_values(DATABASE_INFO_ELEMS),
        service: get_service_status(),
        system: get_system_info(),
    };
    let json = serde_json::ser::to_string(&all_info).unwrap();
    println!("{}", json);
}

/// Prints all property values to stdout as plain text or as json
fn print_config_based_properties(subsubmatches: &ArgMatches, properties: &[&str]) {
    let property_values = get_config_values(properties);
    if subsubmatches.is_present("json") {
        print_as_json(property_values);
        return;
    }
    
    print_as_lines_with_context(property_values, properties, None);
}

/// Returns a hashmap containing the property with a resolved value.
fn get_config_values(properties: &[&str]) -> HashMap<String, Option<String>> {
    let properties_res = get_config_properties(properties);
    if let Err(e) = properties_res {
        print_error(e);
        exit(1);
    }

    properties_res.unwrap()
}

/// Loads service related information.
fn get_system_info() -> HashMap<String, Option<String>> {
    get_system_values(SYSTEM_INFO_ELEMS)
}