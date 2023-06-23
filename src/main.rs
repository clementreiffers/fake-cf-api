use actix_multipart::Multipart;
use actix_web::web::Buf;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::StreamExt;
use tokio::fs::File;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::fs::read_json;
use crate::get_services::{
    handle_accounts_services, handle_memberships, handle_subdomain, handle_user,
};
use crate::post_services::handle_accounts_scripts;

mod fs;
mod get_services;
mod post_services;
mod put_services;

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.next().await {
        let mut field = field.expect("failed to get fields");
        let filename = match field.content_disposition().get_filename() {
            None => "",
            Some(f) => f,
        };
        println!("{}", filename);
        while let Some(chunk) = field.next().await {
            let mut body = web::BytesMut::new();
            let chunk = chunk.expect("failed to get chunk");
            body.extend_from_slice(&chunk);
            let _ = File::create(filename).await?.write_all(body.as_ref()).await;
        }
    }

    Ok(HttpResponse::Ok().body(read_json(
        "./src/defaultResponses/put_accounts_scripts.json",
    )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handle_user)
            .service(handle_memberships)
            .service(handle_subdomain)
            .service(handle_accounts_services)
            .service(handle_accounts_scripts)
            .route(
                "/client/v4/accounts/{accounts}/workers/scripts/{scripts}",
                web::put().to(save_file),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
