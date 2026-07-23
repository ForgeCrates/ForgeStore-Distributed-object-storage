pub mod extras;

use extras::MetadataNodeConfig;
use raft::{Config, raw_node::RawNode, storage::MemStorage};

pub async fn initialize_raft_node(config: MetadataNodeConfig) {
    let raft_config = Config {
        id: config.node_id,
        election_tick: config.election_tick,
        heartbeat_tick: config.heartbeat_tick,
        max_size_per_msg: 1024 * 1024,
        max_inflight_msgs: 256,
        ..Default::default()
    };
    let mut peers=config.peers.clone();
    let mut leader=!config.leader_ip.is_some();  //if no leader self becomes leader
    
    // intialize raft node



    loop {
    ifleader{
        
    } else {
       
    }


    }
}
