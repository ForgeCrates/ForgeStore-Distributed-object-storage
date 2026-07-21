For a distributed object storage system, each service (gateway, metadata, storage node, cluster manager, etc.) is simply an independent Rust binary with its own main.rs. The workspace only groups them together.

1. Workspace Cargo.toml

Your root Cargo.toml should look something like:

[workspace]
resolver = "2"

members = [
    "crates/gateway",
    "crates/metadata",
    "crates/storage-node",
    "crates/cluster-manager",
    "crates/rebalancer",
    "crates/scrubber",
    "crates/garbage-collector",
    "crates/common",
    "crates/protocol",
    "crates/placement",
    "crates/replication",
    "crates/iam",
]
2. Each service is its own executable

For example:

crates/
    metadata/
        Cargo.toml
        src/
            main.rs

    gateway/
        Cargo.toml
        src/
            main.rs

    storage-node/
        Cargo.toml
        src/
            main.rs

Metadata's Cargo.toml

[package]
name = "metadata"
version = "0.1.0"
edition = "2024"

[dependencies]
tokio = { version = "1", features = ["full"] }
common = { path = "../common" }
protocol = { path = "../protocol" }
rocksdb = "0.23"
axum = "0.8"
3. Metadata service

Suppose your main.rs contains

#[tokio::main]
async fn main() {
    println!("Starting metadata service");

    // Load config

    // Open RocksDB

    // Start gRPC server

    // Join raft cluster

    // Wait forever
}

You run it using

cargo run -p metadata

or

cargo run --package metadata
4. Gateway
cargo run -p gateway
5. Storage node
cargo run -p storage-node
6. Running multiple services

Open several terminals.

Terminal 1

cargo run -p metadata

Terminal 2

cargo run -p storage-node

Terminal 3

cargo run -p gateway

Terminal 4

cargo run -p cluster-manager

Now all services are communicating over gRPC/HTTP.

Passing configuration

Instead of hardcoding ports, pass configuration files.

Example

cargo run -p metadata -- \
    --config configs/metadata.toml

or

cargo run -p gateway -- \
    --config configs/gateway.toml

Inside main.rs

let args = Args::parse();

let config =
    Config::load(args.config)?;
Running multiple metadata servers

A distributed system usually has several metadata servers.

Terminal 1

cargo run -p metadata -- \
    --id 1 \
    --config configs/meta1.toml

Terminal 2

cargo run -p metadata -- \
    --id 2 \
    --config configs/meta2.toml

Terminal 3

cargo run -p metadata -- \
    --id 3 \
    --config configs/meta3.toml

Each configuration specifies a different:

node ID
RocksDB path
Raft port
gRPC port

Example:

node_id = 1

grpc = "127.0.0.1:5001"

raft = "127.0.0.1:7001"

db = "./data/meta1"
Running a storage cluster

Likewise, each storage node gets its own configuration.

storage1.toml
storage2.toml
storage3.toml
storage4.toml

Run them:

cargo run -p storage-node -- --config configs/storage1.toml
cargo run -p storage-node -- --config configs/storage2.toml
cargo run -p storage-node -- --config configs/storage3.toml
cargo run -p storage-node -- --config configs/storage4.toml

Now your gateway can distribute objects across these nodes.

Using Docker Compose

Once the services work locally, automate startup with docker-compose.yml:

services:
  metadata1:
    build: .
    command: cargo run -p metadata -- --config configs/meta1.toml

  metadata2:
    build: .
    command: cargo run -p metadata -- --config configs/meta2.toml

  metadata3:
    build: .
    command: cargo run -p metadata -- --config configs/meta3.toml

  storage1:
    build: .
    command: cargo run -p storage-node -- --config configs/storage1.toml

  storage2:
    build: .
    command: cargo run -p storage-node -- --config configs/storage2.toml

  gateway:
    build: .
    command: cargo run -p gateway -- --config configs/gateway.toml

Then start the entire cluster with:

docker compose up
Recommended development workflow

For a production-quality distributed S3 implementation, a common setup is:

Metadata service: 3 Raft nodes (for leader election and fault tolerance)
Storage nodes: 3–10 instances (or more)
Gateway: 1–2 instances behind a load balancer
Cluster manager: 1 instance (or HA pair)
Background services: rebalancer, garbage collector, and scrubber as separate long-running processes

Each service runs as an independent binary, loads its own configuration, listens on its own network ports, and communicates with the others over gRPC or another RPC protocol. This separation makes the system easy to develop locally and straightforward to deploy to Docker, Kubernetes, or physical servers later.