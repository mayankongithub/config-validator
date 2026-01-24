use config_validator::{Host, HostRole, Validate};

#[test]
fn test_valid_ip_address() {
    let host = Host {
        hostname: "server-01".to_string(),
        ip_address: "192.168.1.1".to_string(),
        role: HostRole::Manager,
        enabled: true,
    };
    assert!(host.validate().is_ok());
}

#[test]
fn test_invalid_ip_address() {
    let host = Host {
        hostname: "server-02".to_string(),
        ip_address: "256.1.1.1".to_string(),
        role: HostRole::Storage,
        enabled: true,
    };
    assert!(host.validate().is_err());
}

