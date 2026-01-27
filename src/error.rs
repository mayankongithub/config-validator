use std::fmt;

#[derive(Debug)]
pub enum ValidationError {
    InvalidHostname { hostname: String },
    InvalidIpAddress { ip: String },
    InvalidNetworkInterface { name: String },
    InvalidFilesystemName { name: String, reason: String },
    InvalidMountPoint { path: String, reason: String },
    NoFilesystems,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHostname { hostname } =>
                write!(f, "Invalid hostname: {}", hostname),
            Self::InvalidIpAddress { ip } =>
                write!(f, "Invalid IP address: {}", ip),
            Self::InvalidNetworkInterface { name } =>
                write!(f, "Invalid network interface: {}", name),
            Self::InvalidFilesystemName { name, reason } =>
                write!(f, "Invalid filesystem name '{}': {}", name, reason),
            Self::InvalidMountPoint { path, reason } =>
                write!(f, "Invalid mount point '{}': {}", path, reason),
            Self::NoFilesystems =>
                write!(f, "Storage configuration must have at least one filesystem"),
        }
    }
}

impl std::error::Error for ValidationError {}
