use std::collections::BTreeMap;

use actix_multipart::{Field, Multipart};
use actix_web::web::{Bytes, BytesMut, Path};
use actix_web::{put, Error, HttpResponse};
use futures::StreamExt;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use regex::Regex;
use serde::{Deserialize, Serialize};
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
    !filename.is_empty() && regex.is_match(filename.as_str())
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

fn generate_new_secret_message(secret_name: &String, success: bool) -> serde_json::Value {
    json!({
      "result": {
        "name": secret_name,
        "type": "secret_text"
      },
      "success": success,
      "errors": [],
      "messages": []
    })
}

fn create_secret(
    name: &String,
    text: String,
    secret_type: &String,
) -> k8s_openapi::api::core::v1::Secret {
    let mut string_data = BTreeMap::new();
    string_data.insert(name.into(), text);
    k8s_openapi::api::core::v1::Secret {
        metadata: ObjectMeta {
            name: Some(name.into()),
            ..Default::default()
        },
        string_data: Some(string_data),
        type_: Some(secret_type.into()),
        ..Default::default()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
    text: String,
    #[serde(rename = "type")]
    type_: String,
}

#[put("/client/v4/accounts/{accounts}/workers/scripts/{scripts}/secrets")]
pub async fn new_secret(path: Path<(String, String)>, bytes: Bytes) -> Result<HttpResponse, Error> {
    let (accounts, scripts) = path.into_inner();
    let mut text = String::from_utf8(bytes.to_vec()).expect("failed to cast Bytes to String");
    let values: Data = serde_json::from_str(text.as_mut_str()).expect("failed to deserialize");
    println!("{}", values.type_);
    //let secret = create_secret(&name, text, &secret_type);
    let name = "sg".to_string();
    let message = generate_new_secret_message(&name, true);
    Ok(HttpResponse::Ok().body(message.to_string()))
}
