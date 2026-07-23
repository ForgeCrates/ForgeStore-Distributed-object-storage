// will use grpc here


use protocol::metadata::metadata_service_server::{MetadataService, MetadataServiceServer};
use protocol::metadata::{CreateBucketRequest, CreateBucketResponse /* ... */};
use tonic::{Request, Response, Status};
use crate::bucket_metadata::create_bucket_metadata;
use crate::storage_engine::MetadataDB;
#[derive(Default)]
pub struct MyMetadataService {}

#[tonic::async_trait]
impl MetadataService for MyMetadataService {
    async fn create_bucket(
        &self,
        request: Request<CreateBucketRequest>,
    ) -> Result<Response<CreateBucketResponse>, Status> {
        // Your logic here!
        let response: Result<(), anyhow::Error> = create_bucket_metadata(
            &MetadataDB::new(),
            &request.into_inner().user_id,
            &request.into_inner().bucket_name,
        )?;
        let error_message = match response {
            Ok(_) => "".to_string(),
            Err(e) => e.to_string(),
        };

        let reply = CreateBucketResponse {
            success_message: format!("Created bucket {}", request.into_inner().bucket_name),
            error_message: error_message,
        };
        Ok(Response::new(reply))
    }
    
    // ... implement get_bucket, delete_bucket, create_object
}

// To run the server:
let addr = "[::1]:50051".parse().unwrap();
let service = MyMetadataService::default();
tonic::transport::Server::builder()
    .add_service(MetadataServiceServer::new(service))
    .serve(addr)
    .await?;