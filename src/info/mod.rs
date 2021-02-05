use std::process::{exit, Command};

use clap::ArgMatches;
use crate::config::get_config_properties;


/// Handles all system related commands.
pub fn handle_info_command(submatches: &ArgMatches) {
    if submatches.is_present("database") {
        get_database_info();
        return;
    }

    if submatches.is_present("magicinfo") {
        get_magicinfo_info();
        return;
    }

    if submatches.is_present("service") {
        get_service_info();
        return;
    }

    if submatches.is_present("all") {
        get_all_info();
        return;
    }
}

fn get_all_info() {
    println!("---- MagicINFO ----");
    get_magicinfo_info();
    println!("---- Service ----");
    get_service_info();
    println!("---- Database ----");
    get_database_info();
}

fn get_magicinfo_info() {
    let database_info_elems = &["wsrm.premiumVersion", "CONTENTS_HOME", "web_url"];
    let properties_res = get_config_properties(database_info_elems);
    if let Err(e) = properties_res {
        eprintln!("{}", e);
        exit(1);
    }

    let properties = properties_res.unwrap();

    let version_opt = properties.get("wsrm.premiumVersion").unwrap();
    println!("MagicINFO version:\t{}", version_opt.clone().unwrap());

    let contents_opt = properties.get("CONTENTS_HOME").unwrap();
    println!("Content home:\t\t{}", contents_opt.clone().unwrap());

    let weburl_opt = properties.get("web_url").unwrap();
    println!("Web address:\t\t{}", weburl_opt.clone().unwrap());
}

fn get_database_info() {
    let database_info_elems = &["wsrm.dbVendor", "wsrm.url", "wsrm.username", "wsrm.password"];
    let properties_res = get_config_properties(database_info_elems);
    if let Err(e) = properties_res {
        eprintln!("{}", e);
        exit(1);
    }

    let properties = properties_res.unwrap();

    let vendor_opt = properties.get("wsrm.dbVendor").unwrap();
    println!("Database type:\t\t{}", vendor_opt.clone().unwrap());

    let url_opt = properties.get("wsrm.url").unwrap();
    println!("Database URL:\t\t{}", url_opt.clone().unwrap());

    let username_opt = properties.get("wsrm.username").unwrap();
    println!("Database username:\t{}", username_opt.clone().unwrap());

    let password_opt = properties.get("wsrm.password").unwrap();
    println!("Database password:\t{}", password_opt.clone().unwrap());
}

fn get_service_info() {
    let mut command = Command::new("powershell");
    command.args(&["-c", "$mi = (Get-WmiObject Win32_Service -Filter \"Name='MagicInfoPremium'\"); Write-Host $mi.State; Write-Host $mi.StartName; Write-Host $mi.StartMode;"]);
    let output_res = command.output();
    if output_res.is_err() {
        println!("State:\t\t\t");
        println!("Running as:\t\t");
        println!("Start mode:\t\t");
    } else {
        let start_type = output_res.unwrap().stdout;
        let output = String::from_utf8(start_type).unwrap();
        let output_split: Vec<&str> = output.trim().split("\n").collect();

        println!("State:\t\t\t{}", output_split[0]);
        println!("Running as:\t\t{}", output_split[1]);
        println!("Start mode:\t\t{}", output_split[2]);
    }
}