use config_validator::{
    Validate,
    config::storage::{StorageConfig, Filesystem},
};

#[test]
fn valid_storage_config() {
    let storage = StorageConfig {
        filesystems: vec![
            Filesystem {
                name: "data_disk".to_string(),
                mount_point: "/data".to_string(),
            },
        ],
    };

    assert!(storage.validate().is_ok());
}

#[test]
fn invalid_filesystem_name() {
    let storage = StorageConfig {
        filesystems: vec![
            Filesystem {
                name: "data-disk".to_string(),
                mount_point: "/data".to_string(),
            },
        ],
    };

    assert!(storage.validate().is_err());
}

#[test]
fn invalid_mount_point() {
    let storage = StorageConfig {
        filesystems: vec![
            Filesystem {
                name: "datadisk".to_string(),
                mount_point: "data".to_string(),
            },
        ],
    };

    assert!(storage.validate().is_err());
}

#[test]
fn empty_filesystems_list() {
    let storage = StorageConfig {
        filesystems: vec![],
    };

    assert!(storage.validate().is_err());
}
