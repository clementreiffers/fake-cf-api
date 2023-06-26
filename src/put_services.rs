use actix_multipart::{Field, Multipart};
use actix_web::web::{BytesMut, Json, Path};
use actix_web::{put, Error, HttpResponse};
use futures::StreamExt;
use regex::Regex;
use serde_json::{json, Value};

use crate::fs::read_json;
use crate::upload::upload;

fn get_filename_from_field(field: &Field) -> &str {
    match field.content_disposition().get_filename() {
        None => "",
        Some(f) => f.trim_start_matches("./"),
    }
}

fn is_correct_filename(filename: &String) -> bool {
    let regex = Regex::new(r#"(?i)\.(mjs|js|wasm)$"#).unwrap();
    filename != "" && regex.is_match(filename.as_str())
}

async fn get_file_content(field: &mut Field) -> BytesMut {
    let mut body: BytesMut = BytesMut::new();
    while let Some(chunk) = field.next().await {
        let chunk = chunk.expect("failed to get chunk");
        body.extend_from_slice(&chunk);
    }
    body
}

#[put("/client/v4/accounts/{accounts}/workers/scripts/{scripts}")]
pub async fn save_file(
    mut payload: Multipart,
    path: Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let (accounts, scripts) = path.into_inner();

    while let Some(field) = payload.next().await {
        let mut f = field.expect("failed to get fields");

        let file_content = get_file_content(&mut f).await;
        let path = format!("{}/{}/{}", accounts, scripts, get_filename_from_field(&f));

        if is_correct_filename(&path) {
            upload(&path, file_content).await;
        }
    }

    Ok(HttpResponse::Ok().body(read_json(
        "./src/defaultResponses/put_accounts_scripts.json",
    )))
}

fn generate_new_secret_message(secret_name: String, success: bool) -> serde_json::Value {
    return json!({
      "result": {
        "name": secret_name,
        "type": "secret_text"
      },
      "success": success,
      "errors": [],
      "messages": []
    });
}

#[put("/accounts/{accounts}/workers/scripts/{scripts}/secrets")]
pub async fn new_secret() -> Result<HttpResponse, Error> {
    let secret_name = "TEST".to_owned();

    let message = generate_new_secret_message(secret_name, true);
    Ok(HttpResponse::Ok().body(message.to_string()))
}
