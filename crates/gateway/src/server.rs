use protocol::metadata::metadata_service_client::MetadataServiceClient;
use protocol::metadata::CreateBucketRequest;

// Inside your route handler:
let mut client = MetadataServiceClient::connect("http://[::1]:50051").await?;
let request = tonic::Request::new(CreateBucketRequest {
    user_id: "user123".into(),
    bucket_name: "my-cool-bucket".into(),
});
let response = client.create_bucket(request).await?;