use actix_web::get;
use actix_web::web::Path;
use serde_json::json;

fn generate_new_message(success: bool) -> serde_json::Value {
    json!({
        "result": [
            {
                "name": "TEST",
                "type": "secret_text"
            }
        ],
        "success": success,
        "errors": [],
        "messages": []
    })
}

#[get("/client/v4/accounts/{accounts}/workers/scripts/{scripts}/secrets")]
pub async fn get_secrets_list(path: Path<(String, String)>) -> String {
    let (accounts, scripts) = path.into_inner();

    let label_selector = format!("accounts={},scripts={}", accounts, scripts);

    generate_new_message(true).to_string()
}
