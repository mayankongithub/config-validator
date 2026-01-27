use crate::{Validate, error::ValidationError};

#[derive(Debug)]
pub struct StorageConfig {
    pub filesystems: Vec<Filesystem>,
}

#[derive(Debug)]
pub struct Filesystem {
    pub name: String,
    pub mount_point: String,
}

impl Validate for Filesystem {
    fn validate(&self) -> Result<(), ValidationError> {
        // filesystem name: alphanumeric + underscore, 1–64 chars
        if self.name.is_empty() || self.name.len() > 64 {
            return Err(ValidationError::InvalidFilesystemName {
                name: self.name.clone(),
                reason: "Name must be 1–64 characters".to_string(),
            });
        }

        for c in self.name.chars() {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                return Err(ValidationError::InvalidFilesystemName {
                    name: self.name.clone(),
                    reason: "Only alphanumeric characters and underscore allowed".to_string(),
                });
            }
        }

        // mount point must start with '/'
        if !self.mount_point.starts_with('/') {
            return Err(ValidationError::InvalidMountPoint {
                path: self.mount_point.clone(),
                reason: "Mount point must start with '/'".to_string(),
            });
        }

        Ok(())
    }
}

impl Validate for StorageConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        // must have at least one filesystem
        if self.filesystems.is_empty() {
            return Err(ValidationError::NoFilesystems);
        }

        // validate each filesystem
        for fs in &self.filesystems {
            if let Err(e) = fs.validate() {
                return Err(e);
            }
        }


        Ok(())
    }
}
