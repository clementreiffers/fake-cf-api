use actix_web::{delete, web, HttpResponse};
use serde_json::json;

#[delete("/client/v4/accounts/{accounts}/workers/scripts/{scripts}/secrets/{secrets}")]
pub async fn delete_secrets(path: web::Path<(String, String, String)>) -> HttpResponse {
    let (accounts, scripts, secrets) = path.into_inner();

    let response = json!({
        "result": null,
        "success": true,
        "errors": [],
        "messages": []
    });
    HttpResponse::Ok().body(response.to_string())
}
