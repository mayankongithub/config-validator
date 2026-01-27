use config_validator::{Host, HostRole, Validate};

#[test]
fn valid_host() {
    let host = Host {
        hostname: "manager-01".into(),
        ip_address: "192.168.1.1".into(),
        role: HostRole::Manager,
        enabled: true,
    };
    assert!(host.validate().is_ok());
}

#[test]
fn invalid_hostname() {
    let host = Host {
        hostname: "-bad".into(),
        ip_address: "192.168.1.1".into(),
        role: HostRole::Manager,
        enabled: true,
    };
    assert!(host.validate().is_err());
}

#[test]
fn invalid_ip() {
    let host = Host {
        hostname: "node1".into(),
        ip_address: "999.1.1.1".into(),
        role: HostRole::Client,
        enabled: true,
    };
    assert!(host.validate().is_err());
}
