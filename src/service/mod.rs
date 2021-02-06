mod utils;

use std::{process::exit, thread};

use utils::{ServiceAction, get_status, service_is_available, act_on_service, wait_until, wait_until_available};
use clap::ArgMatches;

/// Handles all system related commands.
pub fn handle_service_command(submatches: &ArgMatches) {
    if submatches.is_present("status") {
        print_status();
        return;
    }

    if let Some(subsubmatches) = submatches.subcommand_matches("start") {
        start_service(subsubmatches.is_present("available"));
        return;
    }

    if submatches.is_present("stop") {
        stop_service();
        return;
    }

    if let Some(subsubmatches) = submatches.subcommand_matches("restart") {
        restart_service(subsubmatches.is_present("available"));
        return;
    }

    if submatches.is_present("available") {
        if service_is_available() {
            println!("Available");
        } else {
            println!("Unavailable");
        }
        return;
    }

    println!("{}", submatches.usage());
}

/// Simply prints the current status of the MagicINFO service.
fn print_status() {
    println!("{}", get_status());
}

/// Starts the MagicINFO service
fn start_service(await_availability: bool) {
    let status = get_status();
    if status != "Stopped" {
        eprint!("The service can only be started if it is currently stopped");
        exit(1);
    }

    act_on_service(ServiceAction::Start);

    println!("Starting MagicINFO...");
    wait_until("Running");
    println!("Service is running.");

    if await_availability {
        println!("Waiting for availibility...");
        wait_until_available();
        println!("Service is available");
    }
}

/// Stops the MagicINFO service
fn stop_service() {
    let status = get_status();
    if status != "Running" {
        eprint!("The service can only be stopped if it is currently running");
        exit(1);
    }

    act_on_service(ServiceAction::Stop);

    println!("Stopping MagicINFO...");
    wait_until("Stopped");
    println!("Service is stopped.");
}

/// Restarts the MagicINFO service
fn restart_service(await_availability: bool) {
    let status = get_status();
    if status != "Running" {
        eprint!("The service can only be restarted if it is currently running");
        exit(1);
    }

    act_on_service(ServiceAction::Restart);

    println!("Restarting MagicINFO...");

    // One second delay to allow the system to go from the Running state to the
    // 'Stop Pending' state.
    thread::sleep(*utils::ONE_SECOND);


    wait_until("Running");

    if await_availability {
        println!("Waiting for availibility...");
        wait_until_available();
        println!("Service is available");
    }
}