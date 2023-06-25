use actix_multipart::Multipart;
use actix_web::web::{Buf, BytesMut};
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::StreamExt;
use regex::Regex;
use tokio::fs::{write, File};
use uuid::Bytes;

use crate::fs::read_json;
use crate::get_services::{
    handle_accounts_services, handle_memberships, handle_subdomain, handle_user,
};
use crate::post_services::handle_accounts_scripts;
use crate::put_services::save_file;

mod fs;
mod get_services;
mod post_services;
mod put_services;

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
