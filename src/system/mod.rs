mod prop;

use clap::ArgMatches;
use serde_json::json;

/// Handles all system related commands.
pub fn handle_system_command(submatches: &ArgMatches) {
    if let Some(subsubmatches) = submatches.subcommand_matches("boardid") {
        if subsubmatches.is_present("json") {
            print_json("boardid", &prop::get_board_id());
            return
        }

        println!("{}", prop::get_board_id());
        return;
    }

    if let Some(subsubmatches) = submatches.subcommand_matches("hwunique") {
        if subsubmatches.is_present("json") {
            print_json("hwunique", &prop::get_hwunique());
            return
        }
        
        println!("{}", prop::get_hwunique());
        return;
    }

    if let Some(subsubmatches) = submatches.subcommand_matches("macaddress") {
        if subsubmatches.is_present("json") {
            print_json("macaddress", &prop::get_mac_address());
            return
        }
        
        println!("{}", prop::get_mac_address());
        return;
    }
}

fn print_json(key: &str, value: &str) {
    println!("{}", json!({
        key: value,
    }));
}