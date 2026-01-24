pub enum NetworkType {
    Ethernet,
    Infiniband,
    OmniPath,
}

pub struct NetworkInterface {
    pub name: String,
    pub network_type: NetworkType,
    pub speed_gbps: u32,
}

pub struct NetworkConfig {
    pub interfaces: Vec<NetworkInterface>,
}
