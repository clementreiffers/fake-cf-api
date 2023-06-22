use std::fs::File;
use std::io::Read;

use actix_web::{get, App, HttpServer, Responder};

fn read_json(path: &str) -> String {
    let mut contents = String::new();
    File::open(path)
        .expect("failed to open file")
        .read_to_string(&mut contents)
        .expect("failed to get content from file");
    contents
}

#[get("/client/v4/user")]
async fn handle_user() -> String {
    format!("done")
}

#[get("/client/v4/memberships")]
async fn handle_memberships() -> String {
    read_json("./src/defaultResponses/get_memberships.json")
}

#[get("/client/v4/accounts/{accounts}/workers/subdomain")]
async fn handle_subdomain() -> String {
    read_json("./src/defaultResponses/get_subdomain.json")
}

#[get("/client/v4/accounts/{accounts}/workers/services/{services}")]
async fn handle_accounts_services() -> String {
    read_json("./src/defaultResponses/get_accounts_services.json")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handle_user)
            .service(handle_memberships)
            .service(handle_subdomain)
            .service(handle_accounts_services)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
