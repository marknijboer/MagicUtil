use std::{io::{Read, Write}, net::TcpStream, process::{Command, Stdio, exit}, thread, time};

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
        eprintln!("{}", e);
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

/// Tests if the service is available by checking the HTTP port. It should return
/// a HTTP response to a HTTP request.
pub fn service_is_available() -> bool {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7001") {
        let result = stream.set_read_timeout(Some(time::Duration::from_secs(1)));
        if let Err(_) = result {
            return false;
        }

        let result = stream.write("GET /MagicInfo/ HTTP/1.1\r\n\r\n".as_bytes());
        if let Err(_) = result {
            return false;
        }

        let mut buffer = String::new();
        let result = stream.read_to_string(&mut buffer);
        if let Err(_) = result {
            return false;
        }

        let bytes = result.unwrap();
        if !(&buffer[..bytes]).starts_with("HTTP") {
            return false;
        }

        return true;
    }

    return false;
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
    let mut command = Command::new("powershell");
    command.args(&["-c", "(Get-WmiObject Win32_Service -Filter \"Name='MagicInfoPremium'\").State"]);
    let output_res = command.output();
    if let Err(e) = output_res {
        eprintln!("{}", e);
        exit(1);
    }
    
    let status = output_res.unwrap().stdout;
    let output = String::from_utf8(status).unwrap();

    return String::from(output.trim());
}