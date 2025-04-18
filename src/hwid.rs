use std::fs;
use std::process;
use winreg::enums::*;
use winreg::RegKey;
use reqwest;
use tokio::fs as tokio_fs;

pub async fn hwid_check() {
    // Define the registry path and key
    let reg_path = r"SYSTEM\CurrentControlSet\Control\IDConfigDB\Hardware Profiles\0001";
    let hwid_key = "HwProfileGuid";

    // Try to read HWID from registry
    let registry_hwid = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(reg_path, KEY_READ)
        .and_then(|key| key.get_value::<String, _>(hwid_key))
        .map(|hwid| hwid.trim().to_string())
        .unwrap_or_else(|_| {
            println!("Failed to read HWID from the registry.");
            process::exit(1);
        });

    println!("Registry HWID: {}", registry_hwid);

    // Fetch the users.txt file from the remote location
    let url = "https://adfs-clickbot.netlify.app/users.txt";
    let response = reqwest::get(url).await.ok()
        .and_then(|r| r.text().await.ok())
        .unwrap_or_else(|| {
            println!("Failed to fetch HWID database from URL.");
            process::exit(1);
        });

    // Debug: Preview the first 150 chars of the fetched file
    println!("Fetched users.txt content snippet: {}", &response[..std::cmp::min(150, response.len())]);

    // Check if the HWID exists in the fetched content (full string match on any line)
    let found = response
        .lines()
        .map(|line| line.trim())  // Clean up spaces around each line
        .any(|line| line == registry_hwid); // Exact match

    if !found {
        println!("HWID NOT found in the database.");
        println!("Exiting application due to unauthorized HWID.");
        process::exit(1);
    }

    println!("HWID is authorized: {}", registry_hwid);
}
