use crate::config::{
    get_config_properties_path,
    get_log_directory
};
use clap::ArgMatches;
use std::process::exit;
use std::process::Command;

enum LogAction {
    Open,
    Tail,
}

pub fn handle_open_command(submatches: &ArgMatches) {
    let file: &str = submatches.value_of("FILE").unwrap();
    if file == "config" {
        open_config_properties();
        return;
    }

    if logfile(LogAction::Open, file) {
        return;
    }

    eprintln!("The requested file could not be found.");
    exit(1);
}

pub fn handle_tail_command(submatches: &ArgMatches) {
    let file: &str = submatches.value_of("FILE").unwrap();
    if logfile(LogAction::Tail, file) {
        return;
    }

    eprintln!("The requested file could not be found.");
    exit(1);
}

fn logfile(action: LogAction, file: &str) -> bool {
    let mut log_path = get_log_directory();
    log_path = log_path.join(file);

    if !log_path.exists() {
        return false;
    }

    match action {
        LogAction::Open =>  {
            open_file(log_path.to_str().unwrap());
        },
        LogAction::Tail => {
            tail_file(log_path.to_str().unwrap());
        }
    }
    return true;
}

fn open_config_properties() {
    let config_path_res = get_config_properties_path();
    if let Err(e) = config_path_res {
        eprintln!("{}", e);
        exit(1);
    }

    let config_path_buf = config_path_res.unwrap();
    let config_path = config_path_buf.to_str().unwrap_or_default();

    open_file(config_path);
}

fn open_file(file: &str) {
    let mut command = Command::new("C:\\Program Files\\Notepad++\\notepad++.exe");
    command.args(&[file]);

    let handle_res = command.spawn();
    if let Err(e) = handle_res {
        eprintln!("{}", e);
        exit(1);
    }

    let handle = handle_res.unwrap();
    println!("Opened file with Notepad++... ({})", handle.id())
}

fn tail_file(file: &str) {
    let file_str = format!("\"{}\"", file);

    let mut command = Command::new("powershell");
    command.args(&["-c", "Get-Content", &file_str, "-Tail", "10", "-Wait"]);

    let handle_res = command.spawn();
    if let Err(e) = handle_res {
        eprintln!("{}", e);
        exit(1);
    }

    let handle = handle_res.unwrap();
    println!("Tailing file... ({})", handle.id())
}