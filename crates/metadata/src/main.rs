// will use grpc here
mod bucket_metadata;
mod storage_engine;
use protocol::metadata::metadata_service_server::{MetadataService, MetadataServiceServer};
// use protocol::metadata::{CreateBucketRequest, CreateBucketResponse, GetBucketRequest, GetBucketResponse,DeleteBucketRequest,DeleteBucketRequest,CreateObjectRequest,CreateObjectResponse};
use crate::bucket_metadata::{create_bucket_metadata,get_bucket_metadata,delete_bucket_metadata};
use crate::storage_engine::MetadataDB;
use protocol::metadata::{
    CreateBucketRequest, CreateBucketResponse, GetBucketRequest, GetBucketResponse,DeleteBucketRequest,DeleteBucketResponse
};
use tonic::{Request, Response, Status};
pub struct MyMetadataService {
    db: MetadataDB,
}

impl MyMetadataService {
    pub fn new() -> Self {
        Self {
            db: MetadataDB::open("metadata.db").expect("Failed to open RocksDB"),
        }
    }
}

#[tonic::async_trait]
impl MetadataService for MyMetadataService {
    async fn create_bucket(
        &self,
        request: Request<CreateBucketRequest>,
    ) -> Result<Response<CreateBucketResponse>, Status> {
        let inner = request.into_inner();
        let response = create_bucket_metadata(&self.db, &inner.user_id, &inner.bucket_name);
        let error_message = match response {
            Ok(_) => "".to_string(),
            Err(e) => e.to_string(),
        };

        let reply = CreateBucketResponse {
            success_message: format!("Created bucket {}", inner.bucket_name),
            error_message,
        };
        Ok(Response::new(reply))
    }

    async fn get_bucket(
        &self,
        request: Request<GetBucketRequest>,
    ) -> Result<Response<GetBucketResponse>, Status> {
         let inner = request.into_inner();
        let response = get_bucket_metadata(&self.db, &inner.user_id, &inner.bucket_name);
        let error_message = match response {
            Ok(_) => "".to_string(),
            Err(e) => e.to_string(),
        };

        let reply = GetBucketResponse {
            success_message: format!("bucket {}", inner.bucket_name),
            error_message,
        };
        Ok(Response::new(reply))

    }

    async fn delete_bucket(
        &self,
        request: Request<DeleteBucketRequest>,
    ) -> Result<Response<DeleteBucketResponse>, Status> {
         let inner = request.into_inner();
        let response = delete_bucket_metadata(&self.db, &inner.user_id, &inner.bucket_name);
        let error_message = match response {
            Ok(_) => "".to_string(),
            Err(e) => e.to_string(),
        };


        let reply = DeleteBucketResponse {
            success_message: format!("bucket deleted: {}", inner.bucket_name),
            error_message,
        };
        Ok(Response::new(reply))

    }
}

// To run the server:
#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();
    let service = MyMetadataService::new();
    tonic::transport::Server::builder()
        .add_service(MetadataServiceServer::new(service))
        .serve(addr)
        .await
        .unwrap();
}
