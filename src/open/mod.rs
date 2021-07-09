mod tail;

use crate::config::{
    get_mi_home_dir,
    get_log_directory,
};
use tail::watch_file;
use clap::ArgMatches;
use std::{path::PathBuf, process::exit};
use std::process::Command;
use std::path::Path;
use std::fs;

const DEFAULT_NOTEPAD_PATH: &str = "C:\\Windows\\System32\\notepad.exe";
const NOTEPAD_PP_PATH: &str = "C:\\Program Files\\Notepad++\\notepad++.exe";

enum LogAction {
    Open,
    Tail,
}

/// Handles all file related commands that requires opening files
/// Handles all file related commands that requires tailing and following files.
pub fn handle_open_command(submatches: &ArgMatches) {
    let file: &str = submatches.value_of("FILE").unwrap();
    handle_command(file, LogAction::Open);
}

/// Handles all file related commands that requires tailing and following files.
pub fn handle_tail_command(submatches: &ArgMatches) {
    let file: &str = submatches.value_of("FILE").unwrap();
    handle_command(file, LogAction::Tail);
}

/// Tries to locate the file and uses the given action to open the file
fn handle_command(file: &str, action: LogAction) {
    if let Some(path) = resolve_file_in_mi_confdir(file) {
        return act_on_file(action, &path);
    }

    if let Some(path) = resolve_file_in_mi_logdir(file) {
        return act_on_file(action, &path);
    }

    if let Some(path) = resolve_file_in_tomcat_confdir(file) {
        return act_on_file(action, &path);
    }

    if let Some(path) = resolve_file_in_tomcat_logdir(file) {
        return act_on_file(action, &path);
    }

    eprintln!("The requested file could not be found.");
    exit(1);
}

/// Takes a valid path and an action, and then performs the action on the path.
fn act_on_file(action: LogAction, path: &str) {
    match action {
        LogAction::Open =>  {
            open_file(path);
        },
        LogAction::Tail => {
            tail_file(path);
        }
    }
}

/// Tries to resolve the file name in the MagicINFO log dir.
fn resolve_file_in_mi_logdir(file: &str) -> Option<String> {
    let mut log_path = get_log_directory();
    log_path = log_path.join(file);

    if !log_path.exists() {
        return None;
    }

    let path_str = log_path.to_str().unwrap();
    Some(String::from(path_str))
}

/// Tries to resolve the file name in the MagicINFO conf dir.
fn resolve_file_in_mi_confdir(file: &str) -> Option<String> {
    let mut conf_path = get_mi_home_dir();

    let mut file_copy = file.clone();
    if file_copy == "conf" {
        file_copy = "config.properties";
    }

    conf_path.push("conf");
    conf_path.push(file_copy);

    if !conf_path.exists() {
        return None;
    }

    let path_str = conf_path.to_str().unwrap();
    Some(String::from(path_str))
}

/// Tries to resolve the file name in the Tomcat conf dir.
fn resolve_file_in_tomcat_confdir(file: &str) -> Option<String> {
    let mut conf_path = get_mi_home_dir();

    conf_path.push("tomcat");
    conf_path.push("conf");
    conf_path.push(file);

    if !conf_path.exists() {
        return None;
    }

    let path_str = conf_path.to_str().unwrap();
    Some(String::from(path_str))
}

/// Tries to resolve the file name in the Tomcat log dir.
fn resolve_file_in_tomcat_logdir(file: &str) -> Option<String> {
    let home_dir = get_mi_home_dir();

    let mut log_path = home_dir.join("tomcat");
    log_path.push("logs");
    log_path.push(file);

    if log_path.exists() {
        let path_str = log_path.to_str().unwrap();
        return Some(String::from(path_str));
    }

    find_stdio_log(file, home_dir)
}

/// Tries to resolve the file as a shortcut for the logs in the Tomcat log dir.
fn find_stdio_log(file: &str, mut home_dir: PathBuf) -> Option<String> {
    let log_part = format!("magicinfopremium-{}.", file);

    home_dir.push("tomcat");
    home_dir.push("logs");

    let paths = fs::read_dir(home_dir.to_str().unwrap()).unwrap();
    let mut matching_paths = Vec::new();

    for path_res in paths {
        let path = path_res.unwrap().path();
        let name = path.display();
        let name_str = name.to_string();
        if name_str.contains(&log_part) {
            matching_paths.push(name_str);
        }
    }

    if matching_paths.len() < 1 {
        return None;
    }

    matching_paths.sort();
    matching_paths.pop()
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
fn get_editor_path<'a>() -> String {
    // Search in the default Notepad++ installation directory.
    if Path::new(NOTEPAD_PP_PATH).exists() {
        return String::from(NOTEPAD_PP_PATH);
    }

    // Use the `where` command to search it in other directories, e.g. installed
    // with scoop.
    if let Some(notepad_pp_path) = resolve_notepad_pp() {
        return notepad_pp_path;
    }

    // Fallback on the default plain Windows editor
    return String::from(DEFAULT_NOTEPAD_PATH);
}

/// Tries to resolve the Notepad++ editor and returns an option with the path.
fn resolve_notepad_pp() -> Option<String> {
    let mut command = Command::new("where");
    command.args(&["notepad++"]);
    let output_res = command.output();

    if output_res.is_err() {
        return None;
    }

    let output = output_res.unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stdout_trimmed = stdout.trim().to_owned();

    if stdout_trimmed.is_empty() {
        return None;
    }

    Some(stdout_trimmed)
}

/// Tails the given file and follows the output
fn tail_file(file: &str) {
    if let Err(e) = watch_file(file) {
        eprintln!("{}", e);
        exit(1);
    }
}