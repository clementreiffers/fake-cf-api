use crate::fs::read_json;
use actix_web::put;

#[put("/client/v4/accounts/{accounts}/workers/scripts/{scripts}")]
pub async fn send_files() -> String {
    read_json("./src/defaultResponses/put_accounts_scripts.json")
}
