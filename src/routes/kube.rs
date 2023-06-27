pub async fn create_kube_client() -> kube::Client {
    kube::Client::try_default()
        .await
        .expect("Failed to create client")
}
