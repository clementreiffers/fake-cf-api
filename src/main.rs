use actix_web::{get, App, HttpResponse, HttpServer};
use k8s_openapi::api::core::v1::Pod;
use kube::api::PostParams;
use kube::Api;
use serde_json::json;

use crate::get_services::{
    handle_accounts_services, handle_memberships, handle_subdomain, handle_user,
};
use crate::post_services::handle_accounts_scripts;
use crate::put_services::{new_secret, save_file};

mod fs;
mod get_services;
mod post_services;
mod put_services;
mod upload;

async fn create_kube_client() -> kube::Client {
    kube::Client::try_default()
        .await
        .expect("Failed to create client")
}

fn create_pod(name: &str, image_name: &str, container_port: u32) -> Pod {
    serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": {
            "name": name
        },
        "spec": {
            "containers": [
                {
                    "name": "example-container",
                    "image": image_name,
                    "ports": [
                        {
                            "containerPort": container_port,
                            "protocol": "TCP"
                        }
                    ]
                }
            ]
        }
    }))
    .expect("Failed to create pod definition")
}

#[get("/new-pod")]
async fn test_kube() -> HttpResponse {
    let client = create_kube_client().await;

    let pod: Pod = create_pod("example-pod", "nginx", 80);

    let pods: Api<Pod> = Api::namespaced(client, "default");
    let _ = pods.create(&PostParams::default(), &pod).await;

    HttpResponse::Ok().body("Pod created!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handle_user)
            .service(handle_memberships)
            .service(handle_subdomain)
            .service(handle_accounts_services)
            .service(handle_accounts_scripts)
            .service(save_file)
            .service(test_kube)
            .service(new_secret)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
