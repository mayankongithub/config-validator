use crate::{Validate, ValidationError};
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Host {
    pub hostname: String,
    pub ip_address: String,
    pub role: HostRole,
    pub enabled: bool,
}

#[derive(Debug, PartialEq)]
pub enum HostRole {
    Manager,
    Storage,
    Client,
    Gateway,
}

impl Validate for Host {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = vec![];

        let h = &self.hostname;
        if h.is_empty()
            || h.len() > 253
            || h.starts_with('-')
            || h.ends_with('-')
            || !h.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            errors.push(ValidationError::InvalidHostname {
                hostname: h.clone(),
                reason: "Invalid hostname format".into(),
            });
        }

        if self.ip_address.parse::<Ipv4Addr>().is_err() {
            errors.push(ValidationError::InvalidIpAddress {
                ip: self.ip_address.clone(),
                reason: "Invalid IPv4 address".into(),
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationError::MultipleErrors(errors))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
