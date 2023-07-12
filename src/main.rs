#![recursion_limit = "256"]

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use clap::Parser;

use routes::get::{get_secrets_list, handle_accounts_services, handle_subdomain, handle_user};
use routes::post::handle_accounts_scripts;
use routes::put::{new_secret, save_file};

use crate::args::S3Params;
use crate::routes::delete::delete_secrets;
use crate::routes::get::{get_base_message, handle_memberships};

mod args;
mod kube_crd;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("listening...");
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(S3Params::parse()))
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
