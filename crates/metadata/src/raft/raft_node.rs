use raft::{
    Config,
    storage::MemStorage,
    raw_node::RawNode,
};
use slog::{Drain, o};

// Select some defaults, then change what we need.
let config = Config {
    id: 1,
    ..Default::default()
};
// Initialize logger.
let logger = slog::Logger::root(slog_stdlog::StdLog.fuse(), o!());
// ... Make any configuration changes.
// After, make sure it's valid!
config.validate().unwrap();
// We'll use the built-in `MemStorage`, but you will likely want your own.
// Finally, create our Raft node!
let storage = MemStorage::new_with_conf_state((vec![1], vec![]));
let mut node = RawNode::new(&config, storage, &logger).unwrap();