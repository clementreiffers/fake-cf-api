use std::collections::BTreeMap;

use actix_web::web::{Bytes, Path};
use actix_web::{put, HttpResponse};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::PostParams;
use kube::Api;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::kube_crd::create_kube_client;

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

fn create_labels(accounts: String, scripts: String, values: &Data) -> BTreeMap<String, String> {
    let mut string_data: BTreeMap<String, String> = BTreeMap::new();
    string_data.insert("accounts".to_string(), accounts);
    string_data.insert("scripts".to_string(), scripts);
    string_data.insert("name".to_string(), values.name.parse().unwrap());
    string_data
}

fn create_secret(
    values: &Data,
    accounts: &String,
    scripts: &String,
) -> k8s_openapi::api::core::v1::Secret {
    k8s_openapi::api::core::v1::Secret {
        string_data: Some(create_string_data(values)),
        type_: Some(values.type_.into()),
        metadata: ObjectMeta {
            name: Option::from(format!("{}.{}.{}", accounts, scripts, values.name)),
            labels: Some(create_labels(accounts.into(), scripts.into(), values)),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[put("/client/v4/accounts/{accounts}/workers/scripts/{scripts}/secrets")]
pub async fn new_secret(path: Path<(String, String)>, bytes: Bytes) -> HttpResponse {
    let (accounts, scripts) = path.into_inner();
    let values: Data = serde_json::from_slice(&bytes).expect("failed to deserialize");
    let secret = create_secret(&values, &accounts, &scripts);

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
