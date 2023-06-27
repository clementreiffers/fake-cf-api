use actix_web::{delete, web, HttpResponse};
use k8s_openapi::api::core::v1::Secret;
use kube::api::{DeleteParams, ListParams};
use kube::Api;
use kube::Client;
use serde_json::json;

use crate::routes::kube::create_kube_client;

#[delete("/client/v4/accounts/{accounts}/workers/scripts/{scripts}/secrets/{secrets}")]
pub async fn delete_secrets(path: web::Path<(String, String, String)>) -> HttpResponse {
    let (accounts, scripts, secrets) = path.into_inner();

    /*    let client = create_kube_client().await;
    let label_selector = format!("accounts={},scripts={}", accounts, scripts);
    let list_params = ListParams::default().labels(&label_selector);
    let kube_secrets: Api<k8s_openapi::api::core::v1::Secret> = Api::all(client);
    let secret_list = kube_secrets.list(&list_params).await;
    let delete_params = DeleteParams::default();

    if let Ok(secret) = secret_list {
        for items in secret.items {
            let name = items.metadata.name.expect("failed to get name");
            if name == secrets {
                match kube_secrets.delete("aaa", &delete_params).await {
                    Ok(_) => println!("deleted"),
                    Err(err) => eprintln!("failed to delete: {}", err),
                }
            }
        }
    }*/
    let client = create_kube_client().await;
    let kube_secret: Api<Secret> = Api::namespaced(client.clone(), "default");
    let label_selector = format!("accounts={},scripts={}", accounts, scripts);
    let list_params = ListParams::default().labels(&label_selector);
    let secrets_list = kube_secret
        .list(&list_params)
        .await
        .expect("failed to get list");

    for items in secrets_list.items {
        let name = items.metadata.name.expect("failed to get name");
        if name == secrets {
            let dp = DeleteParams::default();
            kube_secret
                .delete(&*name, &dp)
                .await
                .expect("failed to delete secret");
        }
    }

    let response = json!({
        "result": null,
        "success": true,
        "errors": [],
        "messages": []
    });
    HttpResponse::Ok().body(response.to_string())
}
