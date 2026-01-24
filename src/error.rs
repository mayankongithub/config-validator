use std::fmt;

#[derive(Debug)]
pub enum ValidationError {
    InvalidIpAddress { ip: String, reason: String },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::InvalidIpAddress { ip, reason } => {
                write!(f, "Invalid IP address '{}': {}", ip, reason)
            }
        }
    }
}

impl std::error::Error for ValidationError {}
