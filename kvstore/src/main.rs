use tokio::net::TcpListener;

use kvstore::server;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", kvstore::DEFAULT_PORT))
        .await
        .unwrap();
    server::run(listener).await;
}
