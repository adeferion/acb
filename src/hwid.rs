#![windows_subsystem = "windows"]

use std::process;
use winreg::enums::*;
use winreg::RegKey;
use reqwest;

pub async fn hwid_check() {
    // Define the registry path and key
    let reg_path = r"SYSTEM\CurrentControlSet\Control\IDConfigDB\Hardware Profiles\0001";
    let hwid_key = "HwProfileGuid";

    // Open the registry key
    let hkcu = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = match hkcu.open_subkey_with_flags(reg_path, KEY_READ) {
        Ok(k) => k,
        Err(_) => {
            println!("failed at index: 14:51, hwid.rs");
            process::exit(1);
        }
    };

    // Read the value of "HwProfileGuid" from the registry
    let registry_hwid: String = match key.get_value(hwid_key) {
        Ok(value) => value,
        Err(_) => {
            println!("failed at index: 24:64, hwid.rs");
            println!("the len is 40185626 but its 40185625");
            process::exit(1);
        }
    };

    // Fetch HWID list from the remote HTML page
    let url = "https://adfs-clickbot.netlify.app/database.html";
    let response = match reqwest::get(url).await {
        Ok(resp) => resp.text().await.unwrap_or_default(),
        Err(_) => {
            println!("failed to fetch HWID database");
            process::exit(1);
        }
    };

    // Check if the registry HWID is in the fetched content
    if !response.contains(&registry_hwid) {
        println!("failed at index: 10:90, hwid.rs");
        println!("the len is 40185627 but its 40185626");
        process::exit(1);
    }

    println!("HWID is authorized: {}", registry_hwid);
}
