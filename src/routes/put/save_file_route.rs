use std::time::{SystemTime, UNIX_EPOCH};

use actix_multipart::{Field, Multipart};
use actix_web::web::{BytesMut, Data, Path};
use actix_web::{put, Error, HttpResponse};
use futures::StreamExt;
use kube::Api;
use regex::Regex;
use rusoto_s3::S3Client;
use serde_json::json;
use tracing::info;

use crate::args::S3Params;
use crate::kube_crd::worker_version::{WorkerVersion, WorkerVersionSpec};
use crate::kube_crd::{
    create_kube_client, kube_create_worker_version, kube_get_worker_version,
    kube_update_worker_version,
};
use crate::routes::put::upload::upload;

fn get_filename_from_field(field: &Field) -> &str {
    match field.content_disposition().get_filename() {
        None => "",
        Some(f) => f.trim_start_matches("./"),
    }
}

fn is_correct_filename(filename: &str) -> bool {
    let regex = Regex::new(r#"(?i)\.(mjs|js|wasm)$"#).unwrap();
    !filename.is_empty() && regex.is_match(filename)
}

async fn get_file_content(field: &mut Field) -> BytesMut {
    let mut body: BytesMut = BytesMut::new();
    while let Some(chunk) = field.next().await {
        let chunk = chunk.expect("failed to get chunk");
        body.extend_from_slice(&chunk);
    }
    body
}

fn generate_message(success: bool) -> serde_json::Value {
    json!({
      "result": {
        "created_on": "2023-04-11T09:04:26.21742Z",
        "modified_on": "2023-06-01T16:45:57.89535Z",
        "id": "hello",
        "tag": "0c2ca8c2d4d44b2490074d7a979899e1",
        "tags": [],
        "deployment_id": "fa2ed69c548e4bd0a742b08e4c43d42b",
        "tail_consumers": null,
        "logpush": false,
        "etag": "f1b79de5c4b1af431ffbd80dcc8883429003af5359704900dd8ee6860aa904f3",
        "handlers": [
          "fetch"
        ],
        "last_deployed_from": "wrangler",
        "compatibility_date": "2023-04-11",
        "usage_model": "bundled",
        "available_on_subdomain": true
      },
      "success": success,
      "errors": [],
      "messages": []
    })
}

#[put("/client/v4/accounts/{accounts}/workers/scripts/{scripts}")]
pub async fn save_file<'a>(
    mut payload: Multipart,
    path: Path<(String, String)>,
    args: Data<S3Params>,
    client: Data<S3Client>,
) -> Result<HttpResponse, Error> {
    let (accounts, scripts) = path.into_inner();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("failed to get timestamp")
        .as_secs();

    let path = format!("{}/{}/{}", &accounts, &scripts, &timestamp);
    while let Some(field) = payload.next().await {
        let mut f = field.expect("failed to get fields");

        let file_content = get_file_content(&mut f).await;
        let filename = get_filename_from_field(&f);
        let file_path = format!("{}/{}", path, filename);
        if is_correct_filename(filename) {
            let is_uploaded = upload(&file_path, file_content, &args, &client).await;
            info!("save file: {}", file_path);
            if !is_uploaded {
                return Ok(HttpResponse::Ok().body(generate_message(false).to_string()));
            }
        }
    }
    let client = create_kube_client().await;

    let worker_version = WorkerVersion::new(
        format!("{}-{}-{}", accounts, scripts, timestamp).as_str(),
        WorkerVersionSpec {
            accounts,
            scripts,
            url: path,
        },
    );

    let api: Api<WorkerVersion> = Api::default_namespaced(client);
    let result: bool = match kube_get_worker_version(&api, &worker_version).await {
        true => kube_update_worker_version(&api, &worker_version).await,
        false => kube_create_worker_version(&api, &worker_version).await,
    };

    println!("all kube resources created? {}", result);

    Ok(HttpResponse::Ok().body(generate_message(result).to_string()))
}
