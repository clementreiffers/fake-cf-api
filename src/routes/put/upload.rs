use std::env;

use actix_web::web::BytesMut;
use dotenv::dotenv;
use rusoto_core::credential::{AwsCredentials, StaticProvider};
use rusoto_core::{ByteStream, HttpClient, Region};
use rusoto_s3::{PutObjectRequest, S3Client, StreamingBody, S3};

fn create_streaming_body(file_content: BytesMut) -> ByteStream {
    let len = file_content.len();
    let stream =
        futures::stream::once(async move { Ok::<_, std::io::Error>(file_content.freeze()) });

    StreamingBody::new(ByteStream::new_with_size(StreamingBody::new(stream), len))
}

pub async fn upload(path: &String, file_content: BytesMut) -> bool {
    dotenv().ok();

    let access_key: String = env::var("ACCESS_KEY").expect("Missing AWS access key ID.");
    let secret_key: String = env::var("SECRET_KEY").expect("Missing AWS secret access key.");
    let name: String = env::var("REGION").expect("Missing AWS Region.");
    let endpoint: String = env::var("ENDPOINT").expect("Missing AWS Endpoint.");
    let bucket: String = env::var("BUCKET_NAME").expect("Missing AWS Bucket Name.");

    let credentials: AwsCredentials = AwsCredentials::new(access_key, secret_key, None, None);
    let provider: StaticProvider = StaticProvider::from(credentials);

    let region: Region = Region::Custom { name, endpoint };
    let dispatcher = HttpClient::new().expect("Failed to create request dispatcher");

    let client: S3Client = S3Client::new_with(dispatcher, provider, region);

    let request = PutObjectRequest {
        bucket,
        key: path.to_owned(),
        body: Some(create_streaming_body(file_content)),
        ..Default::default()
    };
    match client.put_object(request).await {
        Ok(_) => true,
        Err(_) => false,
    }
}
