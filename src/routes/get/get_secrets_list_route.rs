use actix_web::{get, web, HttpResponse};
use kube::api::ListParams;
use kube::Api;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;

use crate::kube_crd::create_kube_client;

#[derive(Serialize, Deserialize)]
struct SecretInfo {
    name: String,
    #[serde(rename = "type")]
    type_: String,
}

#[get("/client/v4/accounts/{accounts}/workers/scripts/{scripts}/secrets")]
pub async fn get_secrets_list(path: web::Path<(String, String)>) -> HttpResponse {
    let (accounts, scripts) = path.into_inner();

    let label_selector = format!("accounts={},scripts={}", accounts, scripts);
    let list_params = ListParams::default().labels(&label_selector);
    let client = create_kube_client().await;
    let secrets: Api<k8s_openapi::api::core::v1::Secret> = Api::all(client);
    let secret_list = secrets.list(&list_params).await;

    let result: Result<Vec<_>, _> = secret_list
        .map(|secret| {
            secret
                .items
                .into_iter()
                .map(|item| SecretInfo {
                    name: item.metadata.name.expect("failed to get name"),
                    type_: item.type_.expect("failed to get type"),
                })
                .collect()
        })
        .map_err(|err| {
            error!("Error retrieving secrets: {:?}", err);
            err
        });

    match result {
        Ok(secrets) => {
            let response = json!({
                "success": true,
                "result": secrets,
            });

            HttpResponse::Ok().body(response.to_string())
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
