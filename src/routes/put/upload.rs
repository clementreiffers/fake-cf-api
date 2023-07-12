use std::env;

use actix_web::web::{BytesMut, Data};
use clap::Parser;
use dotenv::dotenv;
use rusoto_core::credential::{AwsCredentials, StaticProvider};
use rusoto_core::{ByteStream, HttpClient, Region};
use rusoto_s3::{PutObjectRequest, S3Client, StreamingBody, S3};

use crate::args::S3Params;

fn create_streaming_body(file_content: BytesMut) -> ByteStream {
    let len = file_content.len();
    let stream =
        futures::stream::once(async move { Ok::<_, std::io::Error>(file_content.freeze()) });

    StreamingBody::new(ByteStream::new_with_size(StreamingBody::new(stream), len))
}

pub async fn upload<'a>(
    path: &String,
    file_content: BytesMut,
    s3_params: Data<S3Params<'a>>,
) -> bool {
    let region: Region = Region::Custom {
        name: s3_params.s3_region.parse().unwrap(),
        endpoint: s3_params.s3_endpoint.parse().unwrap(),
    };

    let client: S3Client = S3Client::new(region);

    let request = PutObjectRequest {
        bucket: s3_params.s3_bucket_name.parse().unwrap(),
        key: path.to_owned(),
        body: Some(create_streaming_body(file_content)),
        ..Default::default()
    };
    match client.put_object(request).await {
        Ok(_) => true,
        Err(_) => false,
    }
}
