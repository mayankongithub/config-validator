use crate::{Validate, ValidationError};

#[derive(Debug)]
pub struct NetworkConfig {
    pub interfaces: Vec<NetworkInterface>,
}

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
        while i < chars.len() && chars[i].is_ascii_alphabetic() {
            i += 1;
        }

        if i == 0 || i == chars.len() {
            return Err(ValidationError::InvalidNetworkInterface {
                name: name.clone(),
                reason: "Must start with letters and end with digits".into(),
            });
        }

        while i < chars.len() {
            if !chars[i].is_ascii_digit() {
                return Err(ValidationError::InvalidNetworkInterface {
                    name: name.clone(),
                    reason: "Invalid character".into(),
                });
            }
            i += 1;
        }

        Ok(())
    }
}

impl Validate for NetworkConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = vec![];

        for interface in &self.interfaces {
            if let Err(e) = interface.validate() {
                errors.push(e);
            }
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
    fn valid_interface() {
        let ni = NetworkInterface {
            name: String::from("eth0"),
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
}
