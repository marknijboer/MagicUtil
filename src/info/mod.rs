use std::{collections::HashMap, process::{exit}};

use clap::ArgMatches;
use crate::{config::get_config_properties, utils::print_as_lines_with_context};
use crate::system::get_system_values;
use crate::utils::print_as_json;
use crate::service::get_service_status;

const MAGICINFO_INFO_ELEMS: &[&str] = &["wsrm.premiumVersion", "web_url", "CONTENTS_HOME"];
const DATABASE_INFO_ELEMS: &[&str] = &["wsrm.dbVendor", "wsrm.url", "wsrm.username", "wsrm.password"];
const SYSTEM_INFO_ELEMS: &[&str] = &["hwunique", "boardid", "macaddress"];
const SERVICE_INFO_ELEMS: &[&str] = &["state", "serviceUser", "startMode"];

#[derive(Debug, Serialize)]
pub struct AllInfo {
    magicinfo: HashMap<String, Option<String>>,
    database: HashMap<String, Option<String>>,
    system: HashMap<String, Option<String>>,
    service: HashMap<String, Option<String>>,
}


/// Handles all system related commands.
pub fn handle_info_command(submatches: &ArgMatches) {
    if let Some(subsubmatches) = submatches.subcommand_matches("database") {
        if subsubmatches.is_present("json") {
            print_json(get_database_info);
            return;
        }
        
        print_lines(get_database_info, DATABASE_INFO_ELEMS);
        return;
    }

    if let Some(subsubmatches) = submatches.subcommand_matches("magicinfo") {
        if subsubmatches.is_present("json") {
            print_json(get_magicinfo_info);
            return;
        }

        print_lines(get_magicinfo_info, MAGICINFO_INFO_ELEMS);
        return;
    }

    if let Some(subsubmatches) = submatches.subcommand_matches("system") {
        if subsubmatches.is_present("json") {
            print_json(get_system_info);
            return;
        }

        print_lines(get_system_info, SYSTEM_INFO_ELEMS);
        return;
    }

    if let Some(subsubmatches) = submatches.subcommand_matches("service") {
        if subsubmatches.is_present("json") {
            print_json(get_service_status);
        }
        
        print_lines(get_service_status, SERVICE_INFO_ELEMS);
        return;
    }

    if let Some(subsubmatches) = submatches.subcommand_matches("all") {
        let json_output = subsubmatches.is_present("json");
        if json_output {
            print_all_info_as_json();
            return;
        }

        print_all_info_as_lines();
        return;
    }
}

/// Prints all information as plain text
fn print_all_info_as_lines() {
    println!("---- MagicINFO ----");
    print_lines(get_magicinfo_info, MAGICINFO_INFO_ELEMS);
    println!("---- Database ----");
    print_lines(get_database_info, DATABASE_INFO_ELEMS);
    println!("---- System ----");
    print_lines(get_system_info, SYSTEM_INFO_ELEMS);
    println!("---- Service ----");
    print_lines(get_service_status, SERVICE_INFO_ELEMS);
}

/// Prints all information as a json object
fn print_all_info_as_json() {
    let all_info = AllInfo{
        magicinfo: get_magicinfo_info(),
        database: get_database_info(),
        service: get_service_status(),
        system: get_system_info(),
    };
    let json = serde_json::ser::to_string(&all_info).unwrap();
    println!("{}", json);
}

/// Gets MagicINFO related information
fn get_magicinfo_info() -> HashMap<String, Option<String>> {
    let properties_res = get_config_properties(MAGICINFO_INFO_ELEMS);
    if let Err(e) = properties_res {
        eprintln!("{}", e);
        exit(1);
    }

    properties_res.unwrap()
}

/// Returns information about the database.
fn get_database_info() -> HashMap<String, Option<String>> {
    let properties_res = get_config_properties(DATABASE_INFO_ELEMS);
    if let Err(e) = properties_res {
        eprintln!("{}", e);
        exit(1);
    }

    properties_res.unwrap()
}

/// Loads service related information.
fn get_system_info() -> HashMap<String, Option<String>> {
    get_system_values(SYSTEM_INFO_ELEMS)
}

/// Executes the callback and prints the returned values as JSON.
fn print_json<F>(callback: F)
where F: FnOnce() -> HashMap<String, Option<String>> {
    let values = callback();
    print_as_json(values);
}

/// Executes the callback and prints the returned values as plain text.
fn print_lines<F>(callback: F, source: &[&str])
where F: FnOnce() -> HashMap<String, Option<String>> {
    let values = callback();
    print_as_lines_with_context(values, source);
}