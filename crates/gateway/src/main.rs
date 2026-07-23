use protocol::metadata::{
    metadata_service_client::MetadataServiceClient,
    CreateBucketRequest,
    GetBucketRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client =
        MetadataServiceClient::connect("http://[::1]:50051").await?;

    // Create Bucket
    let request = tonic::Request::new(CreateBucketRequest {
        user_id: "user123".to_string(),
        bucket_name: "my-cool-bucket".to_string(),
    });

    let response = client.create_bucket(request).await?;
    println!("Create Bucket Response:");
    println!("{:#?}", response.into_inner());

    // Get Bucket
    let request = tonic::Request::new(GetBucketRequest {
        user_id: "user123".to_string(),
        bucket_name: "my-cool-bucket".to_string(),
    });

    let response = client.get_bucket(request).await?;
    println!("Get Bucket Response:");
    println!("{:#?}", response.into_inner());

    Ok(())
}