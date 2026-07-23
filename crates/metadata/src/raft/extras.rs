use std::env;

pub struct MetadataNodeConfig {
    pub node_id: u64,
    pub address: String,
    pub peers: Vec<String>,
    pub data_dir: String,
    pub election_tick: usize,
    pub heartbeat_tick: usize,
    pub leader_ip: Option<String>,
}

impl MetadataNodeConfig {
    pub fn from_env() -> Self {
        Self {
            node_id: env::var("NODE_ID").unwrap().parse().unwrap(),

            address: env::var("NODE_ADDR").unwrap(),

            peers: env::var("PEERS")
                .unwrap()
                .split(',')
                .map(|s| s.to_string())
                .collect(),

            data_dir: env::var("DATA_DIR").unwrap(),

            election_tick: env::var("ELECTION_TICK").unwrap().parse().unwrap(),

            heartbeat_tick: env::var("HEARTBEAT_TICK").unwrap().parse().unwrap(),
            leader_ip: env::var("LEADER_IP").ok(),
        }
    }
}
