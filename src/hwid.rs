#![windows_subsystem = "windows"]

use std::process;
use winreg::enums::*;
use winreg::RegKey;

pub fn hwid_check() {
    // List of authorized HWIDs
    let hwids = [
        "{59cd1bc0-d793-11ea-bab0-806e6f6e6963}", // Example HWID
        "", // user
        "", // user
    ];

    // Define the registry path and key
    let reg_path = r"SYSTEM\CurrentControlSet\Control\IDConfigDB\Hardware Profiles\0001";
    let hwid_key = "HwProfileGuid";

    // Open the registry key
    let hkcu = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = match hkcu.open_subkey_with_flags(reg_path, KEY_READ) {
        Ok(k) => k,
        Err(_) => {
            println!("failed at index: 16:90, hwid.rs");
            process::exit(1);
        }
    };

    // Read the value of "HwProfileGuid" from the registry
    let registry_hwid: String = match key.get_value(hwid_key) {
        Ok(value) => value,
        Err(_) => {
            println!("failed at index: 17:36, hwid.rs");
            println!("the len is 40185626 but its 40185625");
            process::exit(1);
        }
    };

    // Check if the registry HWID matches any of the authorized HWIDs
    if !hwids.contains(&registry_hwid.as_str()) {
        println!("failed at index: 9:18, hwid.rs");
        println!("the len is 40185627 but its 40185626");
        process::exit(1);
    }

    println!("HWID is authorized: {}", registry_hwid);
}
