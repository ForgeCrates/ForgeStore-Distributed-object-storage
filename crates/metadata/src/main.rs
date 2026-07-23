// will use grpc here
mod bucket_metadata;
mod raft;
mod storage_engine;
mod api;


use self::raft::{initialize_raft_node, extras::MetadataNodeConfig};
use api::MyMetadataService;
use protocol::metadata::metadata_service_server::MetadataServiceServer;


use dotenvy::dotenv;


#[tokio::main]
async fn main() {

   dotenv().ok();
    let config = MetadataNodeConfig::from_env();



    // to communicate with peers
    tokio::spawn(async move {
        initialize_raft_node(config).await;
    });


    // To run the server:
    let addr = "[::1]:50051".parse().unwrap();
    let service = MyMetadataService::new();
    tonic::transport::Server::builder()
        .add_service(MetadataServiceServer::new(service))
        .serve(addr)
        .await
        .unwrap();
    let x = 5;
    println!("{}", x)
}
