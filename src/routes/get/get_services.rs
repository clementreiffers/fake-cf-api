use crate::fs::read_json;
use actix_web::get;

#[get("/client/v4/user")]
pub async fn handle_user() -> String {
    format!("done")
}

#[get("/client/v4/memberships")]
pub async fn handle_memberships() -> String {
    read_json("./src/defaultResponses/get_memberships.json")
}

#[get("/client/v4/accounts/{accounts}/workers/subdomain")]
pub async fn handle_subdomain() -> String {
    read_json("./src/defaultResponses/get_subdomain.json")
}

#[get("/client/v4/accounts/{accounts}/workers/services/{services}")]
pub async fn handle_accounts_services() -> String {
    read_json("./src/defaultResponses/get_accounts_services.json")
}
