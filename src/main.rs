use actix_web::{get, App, HttpResponse, HttpServer};
use k8s_openapi::api::core::v1::Pod;
use kube::api::PostParams;
use kube::Api;
use serde_json::json;

use crate::get_services::{
    handle_accounts_services, handle_memberships, handle_subdomain, handle_user,
};
use crate::post_services::handle_accounts_scripts;
use crate::put_services::{new_secret, save_file};

mod fs;
mod get_services;
mod post_services;
mod put_services;
mod upload;

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
            .service(new_secret)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
