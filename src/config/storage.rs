pub struct StorageConfig {
    pub filesystems: Vec<Filesystem>,
}

pub struct Filesystem {
    pub name: String,
    pub mount_point: String,
}
