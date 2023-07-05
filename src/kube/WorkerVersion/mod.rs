use std::collections::HashMap;

use kube::api::{Patch, PatchParams, PostParams};
use kube::{Api, Client, ResourceExt};
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "api.cf-worker",
    version = "v1",
    kind = "WorkerVersion",
    namespaced
)]
pub struct WorkerVersionSpec {
    accounts: String,
    scripts: String,
    url: String,
}

pub async fn kube_update_worker_version(
    api: &Api<WorkerVersion>,
    worker_version: &WorkerVersion,
) -> bool {
    api.patch(
        &worker_account.name_any(),
        &PatchParams::force(PatchParams::apply("api.cf-worker")),
        &Patch::Apply(&worker_version),
    )
    .await
    .is_ok()
}

pub async fn kube_create_worker_version(
    api: &Api<WorkerVersion>,
    worker_version: &WorkerVersion,
) -> bool {
    api.create(&PostParams::default(), worker_version)
        .await
        .is_ok()
}

pub async fn kube_get_worker_version(
    api: &Api<WorkerVersion>,
    worker_version: &WorkerVersion,
) -> bool {
    api.get(&worker_version.name_any()).await.is_ok()
}

pub fn worker_version_factory(name: &str, spec: WorkerVersionSpec) -> WorkerVersion {
    WorkerVersion::new(name, spec)
}
