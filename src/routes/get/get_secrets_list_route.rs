use actix_web::get;
use actix_web::web::Path;
use kube::api::ListParams;
use kube::{Api, Resource};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::routes::kube::create_kube_client;

fn generate_new_message(success: bool, result: String) -> serde_json::Value {
    json!({
        "result": result,
        "success": success,
        "errors": [],
        "messages": []
    })
}

#[derive(Serialize, Deserialize, Debug)]
struct Labels {
    accounts: String,
    scripts: String,
}

#[get("/client/v4/accounts/{accounts}/workers/scripts/{scripts}/secrets")]
pub async fn get_secrets_list(path: Path<(String, String)>) -> String {
    let (accounts, scripts) = path.into_inner();

    let label_selector = format!("accounts={},scripts={}", accounts, scripts);
    let list_params = ListParams::default().labels(&label_selector);
    let client = create_kube_client().await;
    let secrets: Api<k8s_openapi::api::core::v1::Secret> = Api::all(client);
    let secret_list = secrets.list(&list_params).await;

    let mut result: Vec<serde_json::Value> = vec![];

    if let Ok(secret) = secret_list {
        for items in secret.items {
            let name = format!("{:?}", items.metadata.name.expect("failed to get name"));
            let type_ = format!("{:?}", items.type_.expect("failed to get type"));

            result.push(json!({"name":name, "type":type_}));
        }
    }
    let result = serde_json::to_string(&result).expect("failed to convert result to string");
    generate_new_message(true, result).to_string()
}
