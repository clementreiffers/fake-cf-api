#![recursion_limit = "256"]

use std::sync::Mutex;

use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use clap::Parser;

use routes::get::{get_secrets_list, handle_accounts_services, handle_subdomain, handle_user};
use routes::post::handle_accounts_scripts;
use routes::put::{new_secret, save_file};

use crate::args::{Args, S3Params};
use crate::routes::delete::delete_secrets;
use crate::routes::get::{get_base_message, handle_memberships};

mod args;
mod kube_crd;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("listening...");
    HttpServer::new(|| {
        let args = Args::parse();
        let s3_params = Data::new(Mutex::new(S3Params {
            s3_bucket_name: &args.s3_bucket_name.as_str(),
            s3_endpoint: &*args.s3_endpoint,
            s3_region: &*args.s3_region,
        }));
        App::new()
            .app_data(web::Data::clone(&s3_params))
            .service(get_base_message)
            .service(handle_user)
            .service(handle_memberships)
            .service(handle_subdomain)
            .service(handle_accounts_services)
            .service(handle_accounts_scripts)
            .service(get_secrets_list)
            .service(save_file)
            .service(new_secret)
            .service(delete_secrets)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
