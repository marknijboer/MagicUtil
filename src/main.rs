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
    #[cfg(target_os = "windows")]
    output_vt100::init();

    let matches = cli::match_cli_arguments();

    match matches.subcommand() {
        Some(("system", submatches)) => system::handle_system_command(submatches),
        Some(("config", submatches)) => config::handle_config_command(submatches),
        Some(("open", submatches)) => open::handle_open_command(submatches),
        Some(("tail", submatches)) => open::handle_tail_command(submatches),
        Some(("info", submatches)) => info::handle_info_command(submatches), 
        Some(("service", submatches)) => service::handle_service_command(submatches), 
        Some(("bcrypt", submatches)) => bcrypt::handle_bcrypt_command(submatches),
        _ => {
            unreachable!("No valid subcommand found");
        }
    }
}
