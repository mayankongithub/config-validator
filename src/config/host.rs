use crate::error::ValidationError;
use crate::validation::{Validate, validate_ip_address};

pub enum HostRole {
    Manager,
    Storage,
    Client,
    Gateway,
}

pub struct Host {
    pub hostname: String,
    pub ip_address: String,
    pub role: HostRole,
    pub enabled: bool,
}

impl Validate for Host {
    fn validate(&self) -> Result<(), ValidationError> {
        validate_ip_address(&self.ip_address).map_err(|reason| {
            ValidationError::InvalidIpAddress {
                ip: self.ip_address.clone(),
                reason,
            }
        })
    }
}
