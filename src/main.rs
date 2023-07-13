#![recursion_limit = "256"]

use actix_web::dev::ServiceFactory;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use clap::Parser;
use env_logger::Env;
use rusoto_core::Region;
use rusoto_s3::S3Client;
use tracing_actix_web::TracingLogger;

use routes::get::{get_secrets_list, handle_accounts_services, handle_subdomain, handle_user};
use routes::post::handle_accounts_scripts;
use routes::put::{new_secret, save_file};

use crate::args::S3Params;
use crate::routes::delete::delete_secrets;
use crate::routes::get::{get_base_message, handle_memberships};

mod args;
mod kube_crd;
mod routes;

fn create_s3_client(s3_params: &S3Params) -> S3Client {
    let region: Region = Region::Custom {
        name: s3_params.s3_region.parse().unwrap(),
        endpoint: s3_params.s3_endpoint.parse().unwrap(),
    };
    S3Client::new(region)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        let s3_params: S3Params = S3Params::parse();
        let s3_client: S3Client = create_s3_client(&s3_params);
        App::new()
            .app_data(Data::new(s3_params))
            .app_data(Data::new(s3_client))
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
            .wrap(TracingLogger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
