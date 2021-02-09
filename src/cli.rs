use clap::{Arg, App, SubCommand, ArgMatches};

/// Matches the CLI arguments and returns an object containing the values.
pub fn match_cli_arguments() -> ArgMatches<'static> {
    App::new("MagicINFO Util")
        .version("1.0")
        .about("Useful utilities on a MagicINFO server")
        .subcommand(SubCommand::with_name("system")
            .about("Utilities based on the system itself")
            .arg(Arg::with_name("PROPERTY")
                .multiple(true)
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("json")
                .help("Setting this value returns the property values as json")
                .long("json")
            )
        )
        .subcommand(SubCommand::with_name("info")
            .about("Utilities based on retrieving information from the system")
            .subcommand(SubCommand::with_name("magicinfo")
                .about("Returns MagicINFO related information")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(SubCommand::with_name("database")
                .about("Returns information about the database")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(SubCommand::with_name("service")
                .about("Returns information about MagicINFO service")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(SubCommand::with_name("system")
                .about("Returns information about the system MagicINFO runs on")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(SubCommand::with_name("all")
                .about("Returns all usefull information")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
        )
        .subcommand(SubCommand::with_name("service")
            .about("Utilities based on the MagicINFO Windows service")
            .subcommand(SubCommand::with_name("status")
                .about("Returns the current service status")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the value as json")
                    .long("json")
                ))
            .subcommand(SubCommand::with_name("start")
                .about("Stats the MagicINFO service")
                .arg(Arg::with_name("available")
                    .help("Waits until the HTTP service is available")
                    .long("available")
                )
                .arg(Arg::with_name("silent")
                    .help("Disables output to stdout")
                    .long("silent")
                )
            )
            .subcommand(SubCommand::with_name("stop")
                .about("Stops the MagicINFO service")
                .arg(Arg::with_name("silent")
                    .help("Disables output to stdout")
                    .long("silent")
                )
            )
            .subcommand(SubCommand::with_name("restart")
                .about("Restarts the MagicINFO service")
                .arg(Arg::with_name("available")
                    .help("Waits until the HTTP service is available")
                    .long("available")
                )
                .arg(Arg::with_name("silent")
                    .help("Disables output to stdout")
                    .long("silent")
                )
            )
            .subcommand(SubCommand::with_name("available")
                .about("Checks if the MagicINFO web interface is available")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                )
            )
        )
        .subcommand(SubCommand::with_name("config")
            .about("Returns one or more configuration properties")
            .arg(Arg::with_name("PROPERTY")
                .multiple(true)
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("json")
                .help("Setting this value returns the property values as json")
                .long("json")
            )
        )
        .subcommand(SubCommand::with_name("open")
            .about("Tries to open the given file")
            .arg(Arg::with_name("FILE")
                .required(true)
                .takes_value(true)
            )
        )
        .subcommand(SubCommand::with_name("tail")
            .about("Tries to tail and follow the given file")
            .arg(Arg::with_name("FILE")
                .required(true)
                .takes_value(true)
            )
        )
        .get_matches()
}