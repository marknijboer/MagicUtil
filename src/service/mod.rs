use std::{io::{Read, Write}, net::TcpStream, process::{Command, Stdio, exit}, thread, time};
use clap::ArgMatches;

/// Handles all system related commands.
pub fn handle_service_command(submatches: &ArgMatches) {
    if submatches.is_present("status") {
        get_status();
        return;
    }

    if submatches.is_present("start") {
        let subsubmatches = submatches.subcommand_matches("start").unwrap();
        start(subsubmatches.is_present("available"));
        return;
    }

    if submatches.is_present("stop") {
        stop();
        return;
    }

    if submatches.is_present("restart") {
        let subsubmatches = submatches.subcommand_matches("restart").unwrap();
        restart(subsubmatches.is_present("available"));
        return;
    }

    if submatches.is_present("available") {
        if is_available() {
            println!("Available");
        } else {
            println!("Unavailable");
        }
        return;
    }

    println!("{}", submatches.usage());
}

fn get_status_returned() -> String {
    let mut command = Command::new("powershell");
    command.args(&["-c", "(Get-WmiObject Win32_Service -Filter \"Name='MagicInfoPremium'\").State"]);
    let output_res = command.output();
    if let Err(e) = output_res {
        eprint!("{}", e);
        exit(1);
    }
    
    let status = output_res.unwrap().stdout;
    let output = String::from_utf8(status).unwrap();

    return String::from(output.trim());
}

fn get_status() {
    println!("{}", get_status_returned());
}

fn start(await_availability: bool) {
    let status = get_status_returned();
    if status != "Stopped" {
        eprint!("The service can only be started if it is currently stopped");
        exit(1);
    }

    let mut command = Command::new("powershell");
    command.args(&["-c", "Start-Service", "MagicInfoPremium"]);

    command.stdout(Stdio::null());
    command.stderr(Stdio::null());

    let handle_res = command.spawn();
    if let Err(e) = handle_res {
        eprintln!("{}", e);
        exit(1);
    }

    println!("Starting MagicINFO...");
    let one_second = time::Duration::from_secs(1);
    loop {
        let status = get_status_returned();
        if status == "Running" {
            println!("Service is running.");
            break;
        }

        thread::sleep(one_second);
    }

    if await_availability {
        println!("Waiting for availibility...");
        wait_until_available();
        println!("Service is available");
    }
}

fn stop() {
    let status = get_status_returned();
    if status != "Running" {
        eprint!("The service can only be stopped if it is currently running");
        exit(1);
    }

    let mut command = Command::new("powershell");
    command.args(&["-c", "Stop-Service", "MagicInfoPremium"]);

    command.stdout(Stdio::null());
    command.stderr(Stdio::null());

    let handle_res = command.spawn();
    if let Err(e) = handle_res {
        eprintln!("{}", e);
        exit(1);
    }

    println!("Stopping MagicINFO...");
    let one_second = time::Duration::from_secs(1);
    loop {
        let status = get_status_returned();
        if status == "Stopped" {
            println!("Service is stopped.");
            break;
        }

        thread::sleep(one_second);
    }
}

fn restart(await_availability: bool) {
    let status = get_status_returned();
    if status != "Running" {
        eprint!("The service can only be restarted if it is currently running");
        exit(1);
    }

    let mut command = Command::new("powershell");
    command.args(&["-c", "Restart-Service", "MagicInfoPremium"]);

    command.stdout(Stdio::null());
    command.stderr(Stdio::null());

    let handle_res = command.spawn();
    if let Err(e) = handle_res {
        eprintln!("{}", e);
        exit(1);
    }

    println!("Restarting MagicINFO...");
    let one_second = time::Duration::from_secs(1);
    thread::sleep(one_second);
    loop {
        let status = get_status_returned();
        if status == "Running" {
            println!("Service is restarted.");
            break;
        }

        thread::sleep(one_second);
    }

    if await_availability {
        println!("Waiting for availibility...");
        wait_until_available();
        println!("Service is available");
    }
}

fn wait_until_available() {
    let one_second = time::Duration::from_secs(1);

    loop {
        if is_available() {
            break;
        }
        thread::sleep(one_second);
    }
}

fn is_available() -> bool {
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