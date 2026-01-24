pub fn validate_ip_address(ip: &str) -> Result<(), String> {
    let parts: Vec<&str> = ip.split('.').collect();

    if parts.len() != 4 {
        return Err("IP address must have exactly 4 octets".to_string());
    }

    for part in parts {
        match part.parse::<u8>() {
            Ok(_) => continue,
            Err(_) => return Err(format!("Invalid octet: {}", part)),
        }
    }

    Ok(())
}
