use std::fmt;

#[derive(Debug)]
pub enum ValidationError {
    InvalidHostname { hostname: String, reason: String },
    InvalidIpAddress { ip: String, reason: String },
    InvalidFilesystemName { name: String, reason: String },
    InvalidMountPoint { path: String, reason: String },
    InvalidNetworkInterface { name: String, reason: String },
    InvalidByteSize { input: String, reason: String },
    NoManagerHost,
    NoFilesystems,
    EmptyConfiguration { field: String },
    MultipleErrors(Vec<ValidationError>),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHostname { hostname, reason } => {
                write!(f, "Invalid hostname '{}': {}", hostname, reason)
            }

            Self::InvalidIpAddress { ip, reason } => {
                write!(f, "Invalid IP address '{}': {}", ip, reason)
            }

            Self::InvalidFilesystemName { name, reason } => {
                write!(f, "Invalid filesystem name '{}': {}", name, reason)
            }

            Self::InvalidMountPoint { path, reason } => {
                write!(f, "Invalid mount point '{}': {}", path, reason)
            }

            Self::InvalidNetworkInterface { name, reason } => {
                write!(f, "Invalid network interface '{}': {}", name, reason)
            }

            Self::InvalidByteSize { input, reason } => {
                write!(f, "Invalid byte size '{}': {}", input, reason)
            }

            Self::NoManagerHost => write!(f, "At least one Manager host is required"),

            Self::NoFilesystems => {
                write!(f, "Storage configuration must have at least one filesystem")
            }

            Self::EmptyConfiguration { field } => {
                write!(f, "Configuration field '{}' cannot be empty", field)
            }

            Self::MultipleErrors(errors) => {
                for e in errors {
                    writeln!(f, "{}", e)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ValidationError {}
