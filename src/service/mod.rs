mod service_utils;

pub use service_utils::get_service_status;

use std::{process::exit, thread, fmt::Display};
use serde_json::json;
use service_utils::{ServiceAction, get_status, service_is_available, act_on_service, wait_until, wait_until_available};
use clap::ArgMatches;
use colored::*;

use crate::utils::print_error;

/// Handles all system related commands.
pub fn handle_service_command(submatches: &ArgMatches) {
    match submatches.subcommand() {
        Some(("status", subsubmatches)) => {
            print_status(subsubmatches.get_flag("json"));
        },
        Some(("start", subsubmatches)) => {
            start_service(subsubmatches.get_flag("available"), subsubmatches.get_flag("silent"));
        },
        Some(("stop", subsubmatches)) => {
            stop_service(subsubmatches.get_flag("silent"));
        },
        Some(("restart", subsubmatches)) => {
            restart_service(subsubmatches.get_flag("available"), subsubmatches.get_flag("silent"));
        },
        Some(("available", subsubmatches)) => {
            if subsubmatches.get_flag("json") {
                println!("{}", json!({
                    "available": service_is_available(),
                }));
                return;
            }
    
            let output = if service_is_available() {
                "Available"
            } else {
                "Unavailable"
            };
    
            println!("{}", output);
        },
        Some(("wait", subsubmatches)) => {
            if subsubmatches.get_flag("running") {
                println!("{}", "Waiting until the service is running...".dimmed());
                wait_until("Running");
                println!("{}", "Service is running!".green());
            }

            if subsubmatches.get_flag("available") {
                println!("{}", "Waiting until the service is available...".dimmed());
                wait_until_available();
                println!("{}", "Service is available!".green());
            }
        },
        _ => {
            unreachable!("No valid subcommand found")
        }
    }
}

/// Simply prints the current status of the MagicINFO service.
fn print_status(json_output: bool) {
    if json_output {
        println!("{}", json!({
            "status": get_status(),
        }));
        return;
    }
    
    println!("{}", get_status());
}

/// Starts the MagicINFO service
fn start_service(await_availability: bool, silent: bool) {
    let status = get_status();
    if status != "Stopped" {
        print_error("The service can only be started if it is currently stopped");
        exit(1);
    }

    act_on_service(ServiceAction::Start);

    print("Starting MagicINFO...".dimmed(), silent);
    wait_until("Running");
    print("Service is running!".green(), silent);

    if await_availability {
        print("Waiting for availability...".dimmed(), silent);
        wait_until_available();
        print("Service is available!".green(), silent);
    }
}

/// Stops the MagicINFO service
fn stop_service(silent: bool) {
    let status = get_status();
    if status != "Running" {
        print_error("The service can only be stopped if it is currently running");
        exit(1);
    }

    act_on_service(ServiceAction::Stop);

    print("Stopping MagicINFO...".dimmed(), silent);
    wait_until("Stopped");
    print("Service is stopped!".green(), silent);
}

/// Restarts the MagicINFO service
fn restart_service(await_availability: bool, silent: bool) {
    let status = get_status();
    if status != "Running" {
        print_error("The service can only be restarted if it is currently running");
        exit(1);
    }

    act_on_service(ServiceAction::Restart);

    print("Restarting MagicINFO...".dimmed(), silent);

    // One second delay to allow the system to go from the Running state to the
    // 'Stop Pending' state.
    thread::sleep(*service_utils::ONE_SECOND);


    wait_until("Running");
    print("Service is running!".green(), silent);

    if await_availability {
        print("Waiting for availability...".dimmed(), silent);
        wait_until_available();
        print("Service is available!".green(), silent);
    }
}

fn print(msg: impl Display, silent: bool) {
    if !silent {
        println!("{}", msg);
    }
}