use clap::{Arg, App, ArgMatches, crate_authors, AppSettings};

/// Matches the CLI arguments and returns an object containing the values.
pub fn match_cli_arguments() -> ArgMatches {
    App::new("MagicUtil")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .about("Released under the MIT license.\n\nUseful utilities on a Samsung MagicINFO server for sysadmin tasks.")
        .author(crate_authors!("\n"))
        .subcommand(App::new("system")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Query system properties used in MagicINFO")
            .subcommand(App::new("hwunique")
                .about("Prints the hardware unique calculated from properties of this system")
                .arg(Arg::new("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(App::new("macaddress")
                .about("Prints the system's MAC address")
                .arg(Arg::new("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(App::new("boardid")
                .about("Prints the system's board ID")
                .arg(Arg::new("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(App::new("ipaddress")
                .about("Prints the system's local ipaddress")
                .arg(Arg::new("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
        )
        .subcommand(App::new("info")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Utilities based on retrieving information from the system")
            .subcommand(App::new("magicinfo")
                .about("Returns MagicINFO related information")
                .arg(Arg::new("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(App::new("database")
                .about("Returns information about the database")
                .arg(Arg::new("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(App::new("service")
                .about("Returns information about MagicINFO service")
                .arg(Arg::new("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(App::new("system")
                .about("Returns information about the system MagicINFO runs on")
                .arg(Arg::new("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(App::new("all")
                .about("Returns all usefull information")
                .arg(Arg::new("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
        )
        .subcommand(App::new("service")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Utilities based on the MagicINFO Windows service")
            .subcommand(App::new("status")
                .about("Returns the current service status")
                .arg(Arg::new("json")
                    .help("Setting this value returns the value as json")
                    .long("json")
                ))
            .subcommand(App::new("start")
                .about("Stats the MagicINFO service")
                .arg(Arg::new("available")
                    .help("Waits until the HTTP service is available")
                    .long("available")
                )
                .arg(Arg::new("silent")
                    .help("Disables output to stdout")
                    .long("silent")
                )
            )
            .subcommand(App::new("stop")
                .about("Stops the MagicINFO service")
                .arg(Arg::new("silent")
                    .help("Disables output to stdout")
                    .long("silent")
                )
            )
            .subcommand(App::new("restart")
                .about("Restarts the MagicINFO service")
                .arg(Arg::new("available")
                    .help("Waits until the HTTP service is available")
                    .long("available")
                )
                .arg(Arg::new("silent")
                    .help("Disables output to stdout")
                    .long("silent")
                )
            )
            .subcommand(App::new("available")
                .about("Checks if the MagicINFO web interface is available")
                .arg(Arg::new("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                )
            )
            .subcommand(App::new("wait")
            .setting(AppSettings::ArgRequiredElseHelp)
                .about("Waits until the MagicINFO application is running and accessible.")
                .arg(Arg::new("running")
                    .alias("untilrunning")
                    .help("Waits until the MagicINFO service is running. It might not be available yet when it just started booting.")
                    .long("running")
                )
                .arg(Arg::new("available")
                    .alias("untilavailable")
                    .help("Waits until the MagicINFO service is available and serving via HTTP.")
                    .long("available")
                )
            )
        )
        .subcommand(App::new("config")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Get, set, replace or remove properties from MagicINFO's main config.properties file")
            .subcommand(App::new("get")
                .about("Returns one or more configuration properties")
                .arg(Arg::new("PROPERTY")
                    .multiple_occurrences(true)
                    .required(true)
                    .takes_value(true)
                )
                .arg(Arg::new("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                )
                .arg(Arg::new("decrypt")
                    .help("Setting this flag will try to decrypt the encrypted values")
                    .long("decrypt")
                )
            )
            .subcommand(App::new("set")
                .about("Sets a configuration property")
                .arg(Arg::new("KEY")
                    .required(true)
                    .takes_value(true)
                )
                .arg(Arg::new("VALUE")
                    .required(true)
                    .takes_value(true)
                )
                .arg(Arg::new("encrypt")
                    .help("Setting this flag will encrypt the value before writing it to the config.properties file")
                    .long("encrypt")
                )
            )
            .subcommand(App::new("remove")
                .about("Removes a configuration property")
                .arg(Arg::new("KEY")
                    .required(true)
                    .takes_value(true)
                )
            )
            .subcommand(App::new("replace")
                .about("Changes a configuration property by replacing part of the existing value")
                .arg(Arg::new("KEY")
                    .required(true)
                    .takes_value(true)
                )
                .arg(Arg::new("SEARCH")
                    .required(true)
                    .takes_value(true)
                )
                .arg(Arg::new("REPLACE")
                    .required(true)
                    .takes_value(true)
                )
            )
        )
        .subcommand(App::new("open")
            .about("Tries to open the given file")
            .arg(Arg::new("FILE")
                .required(true)
                .takes_value(true)
            )
        )
        .subcommand(App::new("tail")
            .about("Tries to tail and follow the given file")
            .arg(Arg::new("FILE")
                .required(true)
                .takes_value(true)
            )
        )
        .subcommand(App::new("bcrypt")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Utilities based on MagicINFO's bcrypt hashing algorithm used to store password")
            .subcommand(App::new("hash")
                .about("Hashes the given plaintext with the bcrypt algorithm")
                .arg(Arg::new("PLAINTEXT")
                    .required(true)
                    .takes_value(true)
                )
            )
        )
        .get_matches()
}