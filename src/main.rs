use actix_web::{App, HttpServer};

use routes::get::{
    get_secrets_list, handle_accounts_services, handle_memberships, handle_subdomain, handle_user,
};
use routes::post::handle_accounts_scripts;
use routes::put::{new_secret, save_file};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handle_user)
            .service(handle_memberships)
            .service(handle_subdomain)
            .service(handle_accounts_services)
            .service(handle_accounts_scripts)
            .service(get_secrets_list)
            .service(save_file)
            .service(new_secret)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
