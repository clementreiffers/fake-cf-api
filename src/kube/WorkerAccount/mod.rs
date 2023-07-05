use std::collections::HashMap;

use kube::api::{Patch, PatchParams, PostParams};
use kube::{Api, Client, ResourceExt};
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
struct WorkerAccountMatchLabels {
    #[serde(rename = "matchLabels")]
    match_labels: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
struct WorkerAccountPodTemplate {
    #[serde(rename = "imagePullSecret")]
    image_pull_secret: String,
}

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "api.cf-worker",
    version = "v1",
    kind = "WorkerAccount",
    namespaced
)]
pub struct WorkerAccountSpec {
    #[serde(rename = "workerBundleName")]
    worker_bundle_name: String,
    #[serde(rename = "workerReleaseSelector")]
    worker_release_selector: WorkerAccountMatchLabels,
    #[serde(rename = "podTemplate")]
    pod_template: WorkerAccountPodTemplate,
}

pub async fn kube_update_worker_account(
    api: &Api<WorkerAccount>,
    worker_account: &WorkerAccount,
) -> bool {
    api.patch(
        &worker_account.name_any(),
        &PatchParams::force(PatchParams::apply("api.cf-worker")),
        &Patch::Apply(&worker_account),
    )
    .await
    .is_ok()
}

pub async fn kube_create_worker_account(
    api: &Api<WorkerAccount>,
    worker_account: &WorkerAccount,
) -> bool {
    api.create(&PostParams::default(), worker_account)
        .await
        .is_ok()
}

pub async fn kube_get_worker_account(
    api: &Api<WorkerAccount>,
    worker_account: &WorkerAccount,
) -> bool {
    api.get(&worker_account.name_any()).await.is_ok()
}

pub fn worker_account_factory(name: &str, spec: WorkerAccountSpec) -> WorkerAccount {
    WorkerAccount::new(name, spec)
}
