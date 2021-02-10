mod prop;

pub use prop::get_config_properties_path;
pub use prop::get_mi_home_dir;
pub use prop::get_config_properties;

use clap::ArgMatches;
use std::{path::PathBuf, process::exit};

use crate::utils::{print_as_json, print_as_lines};

const LOG_PROPERTY: &str = "log4j.appender.file.File";

/// Returns the configuration values in the order in which the properties are
/// requested
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
    if submatches.is_present("json") {
        print_as_json(property_values);
        return;
    }

    print_as_lines(property_values, &properties);
    return;
}

/// Returns the log directory path
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