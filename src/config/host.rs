use crate::{Validate, error::ValidationError};
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Host {
    pub hostname: String,
    pub ip_address: String,
    pub role: HostRole,
    pub enabled: bool,
}

#[derive(Debug)]
pub enum HostRole {
    Manager,
    Storage,
    Client,
    Gateway,
}

impl Validate for Host {
    fn validate(&self) -> Result<(), ValidationError> {
        let h = &self.hostname;

        if h.is_empty()
            || h.len() > 253
            || h.starts_with('-')
            || h.ends_with('-')
            || !h.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return Err(ValidationError::InvalidHostname {
                hostname: h.clone(),
            });
        }

        if self.ip_address.parse::<Ipv4Addr>().is_err() {
            return Err(ValidationError::InvalidIpAddress {
                ip: self.ip_address.clone(),
            });
        }

        Ok(())
    }
}
