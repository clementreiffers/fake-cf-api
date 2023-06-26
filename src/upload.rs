use actix_web::web::{Bytes, BytesMut};
use rusoto_core::{ByteStream, Region};
use rusoto_s3::{PutObjectRequest, S3Client, StreamingBody, S3};

fn create_streaming_body(file_content: BytesMut) -> ByteStream {
    let len = file_content.len();
    let stream =
        futures::stream::once(async move { Ok::<_, std::io::Error>(file_content.freeze()) });

    StreamingBody::new(ByteStream::new_with_size(StreamingBody::new(stream), len))
}

pub async fn upload(path: &String, file_content: BytesMut) {
    let region: Region = Region::Custom {
        name: "fr-par".to_owned(),
        endpoint: "s3.fr-par.scw.cloud".to_owned(),
    };

    let client: S3Client = S3Client::new(region);
    let bucket_name = "stage-cf-worker";

    let request = PutObjectRequest {
        bucket: bucket_name.to_owned(),
        key: path.to_owned(),
        body: Some(create_streaming_body(file_content)),
        ..Default::default()
    };

    match client.put_object(request).await {
        Ok(_) => {
            println!("{path} upload succeed !");
        }
        Err(e) => {
            eprintln!("Error while uploading {path} : {:?}", e);
        }
    }
}
