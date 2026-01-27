use config_validator::{NetworkInterface, NetworkType, Validate};

#[test]
fn valid_interface() {
    let ni = NetworkInterface {
        name: "eth0".into(),
        network_type: NetworkType::Ethernet,
        speed_gbps: 10,
    };
    assert!(ni.validate().is_ok());
}

#[test]
fn invalid_interface() {
    let ni = NetworkInterface {
        name: "0eth".into(),
        network_type: NetworkType::Ethernet,
        speed_gbps: 10,
    };
    assert!(ni.validate().is_err());
}
