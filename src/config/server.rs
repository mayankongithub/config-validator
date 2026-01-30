use crate::{Host, HostRole, NetworkConfig, StorageConfig, Validate, ValidationError};

#[derive(Debug)]
pub struct ServerConfig {
    pub name: String,
    pub hosts: Vec<Host>,
    pub storage: StorageConfig,
    pub network: NetworkConfig,
}

impl Validate for ServerConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = vec![];

        // Check for at least one Manager host
        let has_manager = self.hosts.iter().any(|h| h.role == HostRole::Manager);
        if !has_manager {
            errors.push(ValidationError::NoManagerHost);
        }

        // Validate all hosts
        for host in &self.hosts {
            if let Err(e) = host.validate() {
                errors.push(e);
            }
        }

        // Validate storage
        if let Err(e) = self.storage.validate() {
            errors.push(e);
        }

        // Validate network
        if let Err(e) = self.network.validate() {
            errors.push(e);
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
    use crate::{ByteSize, Filesystem, NetworkInterface, NetworkType};

    #[test]
    fn valid_server_config() {
        let config = ServerConfig {
            name: "production-cluster".into(),
            hosts: vec![Host {
                hostname: "manager-01".into(),
                ip_address: "192.168.1.10".into(),
                role: HostRole::Manager,
                enabled: true,
            }],
            storage: StorageConfig {
                filesystems: vec![Filesystem {
                    name: "data_disk".into(),
                    mount_point: "/data".into(),
                    size: ByteSize(1024),
                }],
                default_size: ByteSize(1024),
            },
            network: NetworkConfig {
                interfaces: vec![NetworkInterface {
                    name: "eth0".into(),
                    network_type: NetworkType::Ethernet,
                    speed_gbps: 10,
                }],
            },
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn no_manager_host() {
        let config = ServerConfig {
            name: "test-cluster".into(),
            hosts: vec![Host {
                hostname: "storage-01".into(),
                ip_address: "192.168.1.20".into(),
                role: HostRole::Storage,
                enabled: true,
            }],
            storage: StorageConfig {
                filesystems: vec![Filesystem {
                    name: "data_disk".into(),
                    mount_point: "/data".into(),
                    size: ByteSize(1024),
                }],
                default_size: ByteSize(1024),
            },
            network: NetworkConfig {
                interfaces: vec![NetworkInterface {
                    name: "eth0".into(),
                    network_type: NetworkType::Ethernet,
                    speed_gbps: 10,
                }],
            },
        };

        assert!(config.validate().is_err());
    }
}
