use crate::{Validate, error::ValidationError};

#[derive(Debug)]
pub struct NetworkInterface {
    pub name: String,
    pub network_type: NetworkType,
    pub speed_gbps: u32,
}

#[derive(Debug)]
pub enum NetworkType {
    Ethernet,
    Infiniband,
    OmniPath,
}

impl Validate for NetworkInterface {
    fn validate(&self) -> Result<(), ValidationError> {
        let name = &self.name;
        let chars: Vec<char> = name.chars().collect();

        let mut i = 0;

        // Step 1: must start with letters
        while i < chars.len() && chars[i].is_ascii_alphabetic() {
            i += 1;
        }

        // Step 2: must have at least one letter and one digit
        if i == 0 || i == chars.len() {
            return Err(ValidationError::InvalidNetworkInterface {
                name: name.clone(),
            });
        }

        // Step 3: remaining characters must be digits
        while i < chars.len() {
            if !chars[i].is_ascii_digit() {
                return Err(ValidationError::InvalidNetworkInterface {
                    name: name.clone(),
                });
            }
            i += 1;
        }

        Ok(())
    }
}

