#[cfg(not(windows))]
use std::process::exit;
#[cfg(windows)]
#[macro_use] 
extern crate lazy_static;

#[cfg(windows)]
#[macro_use]
extern crate serde;

mod system;
mod config;

#[cfg(windows)]
mod service;
mod cli;

#[cfg(windows)]
mod open;

#[cfg(windows)]
mod info;
mod utils;
mod encrypt;

fn main() {
    // System subcommand
    let matches = cli::match_cli_arguments();
    if let Some(submatches) = matches.subcommand_matches("system") {
        return system::handle_system_command(submatches);
    }

    // Config subcommand
    if let Some(submatches) = matches.subcommand_matches("config") {
        return config::handle_config_command(submatches);
    }

    // Open subcommand
    if let Some(submatches) = matches.subcommand_matches("open") {
        #[cfg(windows)]
        return open::handle_open_command(submatches);

        #[cfg(not(windows))]
        only_implemented_on_windows();
    }

    // Tail subcommand
    if let Some(submatches) = matches.subcommand_matches("tail") {
        #[cfg(windows)]
        return open::handle_tail_command(submatches);

        #[cfg(not(windows))]
        only_implemented_on_windows();
    }

    // Info subcommand
    if let Some(submatches) = matches.subcommand_matches("info") {
        #[cfg(windows)]
        return info::handle_info_command(submatches);

        #[cfg(not(windows))]
        only_implemented_on_windows();
    }

    // Service subcommand
    if let Some(submatches) = matches.subcommand_matches("service") {
        #[cfg(windows)]
        return service::handle_service_command(submatches);

        #[cfg(not(windows))]
        only_implemented_on_windows();
    }

    println!("{}", matches.usage());
}

#[cfg(not(windows))]
fn only_implemented_on_windows() {
    eprintln!("This function is only implemented on Windows");
    exit(1);
}