use actix_web::{App, HttpServer};

use put::{new_secret, save_file};

mod fs;
mod get_services;
mod post_services;
mod put;
mod upload;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_services::handle_user)
            .service(get_services::handle_memberships)
            .service(get_services::handle_subdomain)
            .service(get_services::handle_accounts_services)
            .service(post_services::handle_accounts_scripts)
            .service(save_file)
            .service(new_secret)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
