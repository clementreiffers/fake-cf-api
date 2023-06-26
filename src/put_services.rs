use std::collections::BTreeMap;

use actix_multipart::{Field, Multipart};
use actix_web::web::{Bytes, BytesMut, Path};
use actix_web::{put, Error, HttpResponse};
use futures::StreamExt;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::PostParams;
use kube::Api;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

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

fn generate_new_secret_message(secret_name: &str, success: bool) -> serde_json::Value {
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

#[derive(Serialize, Deserialize, Debug)]
struct Data<'a> {
    #[serde(borrow)]
    name: &'a str,
    #[serde(borrow)]
    text: &'a str,
    #[serde(rename = "type", borrow)]
    type_: &'a str,
}

fn create_string_data(values: &Data) -> BTreeMap<String, String> {
    let mut string_data: BTreeMap<String, String> = BTreeMap::new();
    string_data.insert(values.name.parse().unwrap(), values.text.parse().unwrap());
    string_data
}

fn create_secret(values: &Data) -> k8s_openapi::api::core::v1::Secret {
    k8s_openapi::api::core::v1::Secret {
        string_data: Some(create_string_data(&values)),
        type_: Some(values.type_.into()),
        metadata: ObjectMeta {
            name: Some(values.name.into()),
            ..Default::default()
        },
        ..Default::default()
    }
}

async fn create_kube_client() -> kube::Client {
    kube::Client::try_default()
        .await
        .expect("Failed to create client")
}

#[put("/client/v4/accounts/{accounts}/workers/scripts/{scripts}/secrets")]
pub async fn new_secret(path: Path<(String, String)>, bytes: Bytes) -> HttpResponse {
    let (accounts, scripts) = path.into_inner();
    let values: Data = serde_json::from_slice(&bytes).expect("failed to deserialize");
    let secret = create_secret(&values);

    let client = create_kube_client().await;

    let pods: Api<k8s_openapi::api::core::v1::Secret> = Api::namespaced(client, "default");
    let result = pods.create(&PostParams::default(), &secret).await.is_ok();

    let message = generate_new_secret_message(values.name, result);

    match result {
        true => println!("adding secrets for accounts {accounts}.{scripts} done!"),
        false => println!("cannot adding secrets for {accounts}.{scripts} failed"),
    };
    HttpResponse::Ok().body(message.to_string())
}
