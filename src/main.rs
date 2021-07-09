#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde;

mod system;
mod config;
mod service;
mod cli;
mod open;
mod info;
mod utils;
mod bcrypt;

fn main() {
    // System subcommand
    let matches = cli::match_cli_arguments();

    match matches.subcommand() {
        ("system", Some(submatches)) => system::handle_system_command(submatches),
        ("config", Some(submatches)) => config::handle_config_command(submatches),
        ("open", Some(submatches)) => open::handle_open_command(submatches),
        ("tail", Some(submatches)) => open::handle_tail_command(submatches),
        ("info", Some(submatches)) => info::handle_info_command(submatches), 
        ("service", Some(submatches)) => service::handle_service_command(submatches), 
        ("bcrypt", Some(submatches)) => bcrypt::handle_bcrypt_command(submatches),
        _ => println!("{}", matches.usage())
    }
}
