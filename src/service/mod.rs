mod service_utils;

pub use service_utils::get_service_status;

use std::{process::exit, thread};
use serde_json::json;
use service_utils::{ServiceAction, get_status, service_is_available, act_on_service, wait_until, wait_until_available};
use clap::ArgMatches;

use crate::utils::print_error;

/// Handles all system related commands.
pub fn handle_service_command(submatches: &ArgMatches) {
    match submatches.subcommand() {
        Some(("status", subsubmatches)) => {
            print_status(subsubmatches.is_present("json"));
        },
        Some(("start", subsubmatches)) => {
            start_service(subsubmatches.is_present("available"), subsubmatches.is_present("silent"));
        },
        Some(("stop", subsubmatches)) => {
            stop_service(subsubmatches.is_present("silent"));
        },
        Some(("restart", subsubmatches)) => {
            restart_service(subsubmatches.is_present("available"), subsubmatches.is_present("silent"));
        },
        Some(("available", subsubmatches)) => {
            if subsubmatches.is_present("json") {
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
            if subsubmatches.is_present("untilrunning") {
                println!("Waiting until the service is running...");
                wait_until("Running");
                println!("Service is running!");
            }

            if subsubmatches.is_present("untilavailable") {
                println!("Waiting until the service is available...");
                wait_until_available();
                println!("Service is available!");
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

    print("Starting MagicINFO...", silent);
    wait_until("Running");
    print("Service is running!", silent);

    if await_availability {
        print("Waiting for availability...", silent);
        wait_until_available();
        print("Service is available!", silent);
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

    print("Stopping MagicINFO...", silent);
    wait_until("Stopped");
    print("Service is stopped!", silent);
}

/// Restarts the MagicINFO service
fn restart_service(await_availability: bool, silent: bool) {
    let status = get_status();
    if status != "Running" {
        print_error("The service can only be restarted if it is currently running");
        exit(1);
    }

    act_on_service(ServiceAction::Restart);

    print("Restarting MagicINFO...", silent);

    // One second delay to allow the system to go from the Running state to the
    // 'Stop Pending' state.
    thread::sleep(*service_utils::ONE_SECOND);


    wait_until("Running");
    print("Service is running!", silent);

    if await_availability {
        print("Waiting for availability...", silent);
        wait_until_available();
        print("Service is available!", silent);
    }
}

fn print(msg: &str, silent: bool) {
    if !silent {
        println!("{}", msg);
    }
}