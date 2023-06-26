use actix_web::{App, HttpServer};
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};

use crate::get_services::{
    handle_accounts_services, handle_memberships, handle_subdomain, handle_user,
};
use crate::post_services::handle_accounts_scripts;
use crate::put_services::save_file;

mod fs;
mod get_services;
mod post_services;
mod put_services;
/*
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handle_user)
            .service(handle_memberships)
            .service(handle_subdomain)
            .service(handle_accounts_services)
            .service(handle_accounts_scripts)
            .service(save_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
*/
#[tokio::main]
async fn main() {
    let region: Region = Region::Custom {
        name: "fr-par".to_owned(),
        endpoint: "s3.fr-par.scw.cloud".to_owned(),
    };

    let client: S3Client = S3Client::new(region);
    let bucket_name = "stage-cf-worker";

    let file_content: Vec<u8> = vec![
        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x52, 0x75, 0x73, 0x74, 0x21,
    ];

    let request = PutObjectRequest {
        bucket: bucket_name.to_owned(),
        key: "gros/g.txt".to_owned(),
        body: Some(file_content.into()),
        ..Default::default()
    };

    match client.put_object(request).await {
        Ok(_) => {
            println!("File upload succeed !");
        }
        Err(e) => {
            eprintln!("Error while uploading file : {:?}", e);
        }
    }
}
