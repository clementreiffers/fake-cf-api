mod fs;
mod get_services;
mod post_services;
mod put_services;

use crate::get_services::{
    handle_accounts_services, handle_memberships, handle_subdomain, handle_user,
};
use crate::post_services::handle_accounts_scripts;
use crate::put_services::send_files;
use actix_web::{App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handle_user)
            .service(handle_memberships)
            .service(handle_subdomain)
            .service(handle_accounts_services)
            .service(handle_accounts_scripts)
            .service(send_files)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
