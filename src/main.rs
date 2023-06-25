use actix_multipart::{Field, Multipart};
use actix_web::web::{Buf, BytesMut};
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::StreamExt;
use regex::Regex;
use tokio::fs::{write, File};
use tokio::io::{AsyncWrite, AsyncWriteExt};
use uuid::Bytes;

use crate::fs::read_json;
use crate::get_services::{
    handle_accounts_services, handle_memberships, handle_subdomain, handle_user,
};
use crate::post_services::handle_accounts_scripts;

mod fs;
mod get_services;
mod post_services;
mod put_services;

fn get_filename_from_field(field: &Field) -> &str {
    match field.content_disposition().get_filename() {
        None => "",
        Some(f) => f,
    }
}

fn is_correct_filename(filename: &str) -> bool {
    let regex = Regex::new(r#"(?i)\.(mjs|js|wasm)$"#).unwrap();
    filename != "" && regex.is_match(filename)
}

async fn get_file_content(field: &mut Field) -> BytesMut {
    let mut body: BytesMut = BytesMut::new();
    while let Some(chunk) = field.next().await {
        let chunk = chunk.expect("failed to get chunk");
        body.extend_from_slice(&chunk);
    }
    body
}

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(field) = payload.next().await {
        let mut f = field.expect("failed to get fields");

        let file_content: BytesMut = get_file_content(&mut f).await;
        let filename: &str = get_filename_from_field(&f);

        if is_correct_filename(filename) {
            write(filename, &file_content).await?;
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
