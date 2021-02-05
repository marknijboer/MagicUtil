use clap::ArgMatches;

mod prop;

/// Handles all system related commands.
pub fn handle_system_command(submatches: &ArgMatches) {
    if submatches.is_present("boardid") {
        println!("{}", prop::get_board_id());
        return;
    }

    if submatches.is_present("hwunique") {
        println!("{}", prop::get_hwunique());
        return;
    }

    if submatches.is_present("macaddress") {
        println!("{}", prop::get_mac_address());
        return;
    }
}