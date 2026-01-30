use crate::{ByteSize, Validate, ValidationError};

#[derive(Debug)]
pub struct StorageConfig {
    pub filesystems: Vec<Filesystem>,
    pub default_size: ByteSize,
}

#[derive(Debug)]
pub struct Filesystem {
    pub name: String,
    pub mount_point: String,
    pub size: ByteSize,
}

impl Validate for Filesystem {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = vec![];

        // filesystem name: alphanumeric + underscore, 1–64 chars
        if self.name.is_empty() || self.name.len() > 64 {
            errors.push(ValidationError::InvalidFilesystemName {
                name: self.name.clone(),
                reason: "Name must be 1–64 characters".to_string(),
            });
        }

        if !self
            .name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            errors.push(ValidationError::InvalidFilesystemName {
                name: self.name.clone(),
                reason: "Only alphanumeric characters and underscore allowed".to_string(),
            });
        }

        // mount point must start with '/'
        if !self.mount_point.starts_with('/') {
            errors.push(ValidationError::InvalidMountPoint {
                path: self.mount_point.clone(),
                reason: "Mount point must start with '/'".to_string(),
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationError::MultipleErrors(errors))
        }
    }
}

impl Validate for StorageConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        // must have at least one filesystem
        if self.filesystems.is_empty() {
            return Err(ValidationError::NoFilesystems);
        }

        let mut errors = vec![];

        // validate each filesystem
        for fs in &self.filesystems {
            if let Err(e) = fs.validate() {
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
    fn valid_storage_config() {
        let storage = StorageConfig {
            default_size: ByteSize(1024),
            filesystems: vec![Filesystem {
                name: "data_disk".into(),
                mount_point: "/data".into(),
                size: ByteSize(1024),
            }],
        };

        assert!(storage.validate().is_ok());
    }

    #[test]
    fn invalid_filesystem_name() {
        let storage = StorageConfig {
            default_size: ByteSize(1024),
            filesystems: vec![Filesystem {
                name: "data-disk".into(),
                mount_point: "/data".into(),
                size: ByteSize(1024),
            }],
        };

        assert!(storage.validate().is_err());
    }

    #[test]
    fn invalid_mount_point() {
        let storage = StorageConfig {
            default_size: ByteSize(1024),
            filesystems: vec![Filesystem {
                name: "datadisk".into(),
                mount_point: "data".into(),
                size: ByteSize(1024),
            }],
        };

        assert!(storage.validate().is_err());
    }

    #[test]
    fn empty_filesystems_list() {
        let storage = StorageConfig {
            default_size: ByteSize(1024),
            filesystems: vec![],
        };

        assert!(storage.validate().is_err());
    }
}
