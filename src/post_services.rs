use crate::fs::read_json;
use actix_web::post;

#[post("/client/v4/accounts/{accounts}/workers/scripts/{script}/subdomain")]
pub async fn handle_accounts_scripts() -> String {
    read_json("./src/defaultResponses/get_accounts_services.json")
}
