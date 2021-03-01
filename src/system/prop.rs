use mac_address;
use local_ipaddress;
use std::process::Command;
use md5;

const STANDARD_MAC_ADDRESS: &str = "112233AABBCC";
const STANDARD_BOARD_ID: &str = "MI0020130925";

/// Returns the system's hardware unique identifier, which is based on the board
/// ID and the MAC address of this device.
pub fn get_hwunique() -> String {
    let mac = get_mac_address().replace(":", "").to_uppercase();
    let board_id = get_board_id().replace("-", "").to_uppercase();
    let hash = md5::compute((mac + board_id.as_str()).as_str());
    let hash_string = format!("{:X}", hash);

    hash_string.chars().take(16).collect::<String>()
}

/// Returns the MAC address of this device. It will return the same default value
/// as MagicINFO does if the real MAC cannot be found.
pub fn get_mac_address() -> String {
    let mac_opt_res = mac_address::get_mac_address();
    if mac_opt_res.is_err() {
        return String::from(STANDARD_MAC_ADDRESS);
    }

    let mac_opt = mac_opt_res.unwrap();
    if mac_opt.is_none() {
        return String::from(STANDARD_MAC_ADDRESS);
    }
    
    let mac_string = format!("{}", mac_opt.unwrap());
    mac_string.trim().to_owned()
}

/// Returns the system's board ID on a Windows device by calling the inbuilt 
/// `wmic` tool. If it cannot be found for some reason, the same default value
/// will be returned as used by MagicINFO.
pub fn get_board_id() -> String {

    // On Windows, try getting the board ID through the wmic call. Non-windows
    // environment should exit here now.
    if !cfg!(target_os = "windows") {
        return String::from(STANDARD_BOARD_ID);
    }

    // Execute the wmic call to get the board ID.
    let output_res = Command::new("wmic")
        .args(&["bios", "get", "serialnumber"]).output();
    if output_res.is_err() {
        return String::from(STANDARD_BOARD_ID);
    }

    // Try to parse the output as a valid UTF-8 string.
    let output = output_res.unwrap();
    let str_res = String::from_utf8(output.stdout);
    if str_res.is_err() {
        return String::from(STANDARD_BOARD_ID);
    }

    // Parse the board ID by removing the leading `SerialNumber` label and
    // trimming what's left
    let mut str_out = str_res.unwrap();
    str_out = str_out.replace("SerialNumber", "").trim().to_string();
    if str_out.is_empty() {
        return String::from(STANDARD_BOARD_ID);
    }

    str_out.trim().to_owned()
}

pub fn get_ip_address() -> String {
    let ipaddr_opt = local_ipaddress::get();
    if ipaddr_opt.is_some() {
        return ipaddr_opt.unwrap();
    }

    String::new()
}