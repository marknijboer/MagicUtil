use std::{collections::HashMap, net::TcpStream, process::{Command, Stdio, exit}, thread, time};

use crate::utils::{get_wmic_output_as_list, print_error};

lazy_static! {
    pub static ref ONE_SECOND: time::Duration = time::Duration::from_secs(1);
}


pub enum ServiceAction {
    Start,
    Stop,
    Restart
}

/// Converts a service action to the corresponding Powershell command.
pub fn action_to_command(action: ServiceAction) -> &'static str {
    match action {
        ServiceAction::Start => "Start-Service",
        ServiceAction::Stop => "Stop-Service",
        ServiceAction::Restart => "Restart-Service",
    }
}

/// Executes the given action on the service
pub fn act_on_service(action: ServiceAction) {
    let action_cmd = action_to_command(action);

    let mut command = Command::new("powershell");
    command.args(&["-c", action_cmd, "MagicInfoPremium"]);

    command.stdout(Stdio::null());
    command.stderr(Stdio::null());

    let handle_res = command.spawn();
    if let Err(e) = handle_res {
        print_error(e);
        exit(1);
    }
}

/// Loops and sleeps until the service is available
pub fn wait_until_available() {
    loop {
        if service_is_available() {
            return;
        }
        thread::sleep(*ONE_SECOND);
    }
}

/// Tests if the service is available by checking if the FTP port is open for
/// for connections
pub fn service_is_available() -> bool {
    if TcpStream::connect("127.0.0.1:21").is_err() {
        return false;
    }

    true
}

/// Watches the state of the MagicINFO service and returns only if the state
/// matches the one required as the parameter.
pub fn wait_until(state: &str) {
    loop {
        let status = get_status();
        if status == state {
            return;
        }

        thread::sleep(*ONE_SECOND);
    }
}

/// Returns the current status of the MagicINFO service
pub fn get_status() -> String {
    let mut command = Command::new("wmic");
    command.args(&["Service", "WHERE", "name='MagicInfoPremium'", "GET", "State"]);
    let output_res = command.output();
    if let Err(e) = output_res {
        print_error(e);
        exit(1);
    }
    

    let status_output = output_res.unwrap().stdout;
    let output_split = get_wmic_output_as_list(status_output);

    output_split[0].clone()
}

/// Returns information about the service that runs MagicINFO.
pub fn get_service_status() -> HashMap<String, Option<String>> {
    let mut command = Command::new("wmic");
    command.args(&["Service", "WHERE", "name='MagicInfoPremium'", "GET", "State,StartName,StartMode"]);
    
    let mut property_map = HashMap::new();
    let output_res = command.output();
    if output_res.is_err() {
        property_map.insert(String::from("state"), None);
        property_map.insert(String::from("serviceUser"), None);
        property_map.insert(String::from("startMode"), None);
    } else {
        let status_output = output_res.unwrap().stdout;
        let output_split = get_wmic_output_as_list(status_output);

        property_map.insert(String::from("state"), Some(output_split[0].clone()));
        property_map.insert(String::from("serviceUser"), Some(output_split[1].clone()));
        property_map.insert(String::from("startMode"), Some(output_split[2].clone()));
    }

    property_map
}