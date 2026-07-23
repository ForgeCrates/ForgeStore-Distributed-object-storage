use protocol::metadata::metadata_service_client::MetadataServiceClient;
use protocol::metadata::CreateBucketRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MetadataServiceClient::connect("http://[::1]:50051").await?;
    
    let request = tonic::Request::new(CreateBucketRequest {
        user_id: "user123".into(),
        bucket_name: "my-cool-bucket".into(),
    });
    
    let response = client.create_bucket(request).await?;
    println!("{:?}", response.into_inner());
    
    Ok(())
}
