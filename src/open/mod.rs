mod tail;

use crate::config::{
    get_config_properties_path,
    get_log_directory
};
use tail::watch_file;
use clap::ArgMatches;
use std::process::exit;
use std::process::Command;
use std::path::Path;

const DEFAULT_NOTEPAD_PATH: &str = "C:\\Windows\\System32\\notepad.exe";
const NOTEPAD_PP_PATH: &str = "C:\\Program Files\\Notepad++\\notepad++.exe";

enum LogAction {
    Open,
    Tail,
}

/// Handles all file related commands that requires opening files
pub fn handle_open_command(submatches: &ArgMatches) {
    let file: &str = submatches.value_of("FILE").unwrap();
    if file == "config" || file == "config.properties" {
        open_config_properties();
        return;
    }

    if logfile(LogAction::Open, file) {
        return;
    }

    eprintln!("The requested file could not be found.");
    exit(1);
}

/// Handles all file related commands that requires tailing and following files.
pub fn handle_tail_command(submatches: &ArgMatches) {
    let file: &str = submatches.value_of("FILE").unwrap();
    if logfile(LogAction::Tail, file) {
        return;
    }

    eprintln!("The requested file could not be found.");
    exit(1);
}

/// Searches the file in the log directory. If found, it will execute the
/// requested action.
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

/// Opens the config file in the editor
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

/// Opens the given file in the editor
fn open_file(file: &str) {
    let editor = get_editor_path();
    let mut command = Command::new(editor);
    command.args(&[file]);

    let handle_res = command.spawn();
    if let Err(e) = handle_res {
        eprintln!("{}", e);
        exit(1);
    }

    let handle = handle_res.unwrap();
    println!("Opened file with Notepad++... ({})", handle.id())
}

/// Tries to resolve the Notepad++ editor. If not found, it will return the path
/// to the default notepad editor.
fn get_editor_path() -> &'static str {
    if Path::new(NOTEPAD_PP_PATH).exists() {
        return NOTEPAD_PP_PATH;
    }

    return DEFAULT_NOTEPAD_PATH;
}

/// Tails the given file and follows the output
fn tail_file(file: &str) {
    if let Err(e) = watch_file(file) {
        eprintln!("{}", e);
        exit(1);
    }
}