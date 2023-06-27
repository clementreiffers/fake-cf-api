use actix_web::{delete, web, HttpResponse};
use k8s_openapi::api::core::v1::Secret;
use kube::api::{DeleteParams, ListParams};
use kube::Api;
use serde_json::json;

use crate::routes::kube::create_kube_client;

#[delete("/client/v4/accounts/{accounts}/workers/scripts/{scripts}/secrets/{secrets}")]
pub async fn delete_secrets(path: web::Path<(String, String, String)>) -> HttpResponse {
    let (accounts, scripts, secrets) = path.into_inner();
    let secret_name = format!("{}.{}.{}", accounts, scripts, secrets);

    let client = create_kube_client().await;
    let kube_secret: Api<Secret> = Api::namespaced(client.clone(), "default");
    let label_selector = format!("accounts={},scripts={}", accounts, scripts);
    let list_params = ListParams::default().labels(&label_selector);

    let mut success = false;

    if let Ok(secrets_list) = kube_secret.list(&list_params).await {
        if let Some(secret) = secrets_list
            .items
            .into_iter()
            .find(|item| item.metadata.name == Some(secret_name.clone()))
        {
            let dp = DeleteParams::default();
            success = kube_secret
                .delete(&secret.metadata.name.unwrap(), &dp)
                .await
                .is_ok();
        }
    }

    let response = json!({
        "result": null,
        "success": success,
        "errors": [],
        "messages": []
    });

    HttpResponse::Ok().body(response.to_string())
}
