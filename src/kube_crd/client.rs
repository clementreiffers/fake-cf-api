pub async fn create_kube_client() -> kube::Client {
    kube::Client::try_default()
        .await
        .expect("failed to connect to client")
}
