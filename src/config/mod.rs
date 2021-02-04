use clap::ArgMatches;
use std::{path::PathBuf, process::exit};

mod prop;

const LOG_PROPERTY: &str = "log4j.appender.file.File";

pub use prop::get_config_properties_path;

pub fn handle_config_command(submatches: &ArgMatches) {
    let properties: Vec<&str> = submatches.values_of("PROPERTY").unwrap().collect();
    if properties.is_empty() {
        eprintln!("Expected one or more property keys");
        exit(1);
    }

    let property_values_res = prop::get_config_properties(&properties);
    if let Err(e) = property_values_res {
        eprintln!("{}", e);
        exit(1);
    }

    let property_values = property_values_res.unwrap();
    for property in properties {
        let property_value = property_values.get(property).unwrap();
        println!("{}", property_value.clone().unwrap_or_default());
    }

    return;
}

pub fn get_log_directory() -> PathBuf {
    let property_values_res = prop::get_config_properties(&[LOG_PROPERTY]);
    if let Err(e) = property_values_res {
        eprintln!("{}", e);
        exit(1);
    }

    let property_values = property_values_res.unwrap();
    let log_path_opt = property_values.get(LOG_PROPERTY).unwrap().clone();
    if log_path_opt.is_none() {
        eprintln!("The log path is not defined in the config.properties file");
        exit(1);
    }

    let log_path = log_path_opt.unwrap();
    let log_path_buf = PathBuf::from(log_path);
    PathBuf::from(log_path_buf.parent().unwrap())
}