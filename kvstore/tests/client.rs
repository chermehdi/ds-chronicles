use std::net::SocketAddr;
use tokio::net::TcpListener;

use kvstore::protocol::Response;
use kvstore::Result;

#[tokio::test]
async fn test_ping() {
    let addr = start_server().await.unwrap();
    let mut client = kvstore::client::create(addr).await.unwrap();
    let res = client.ping(String::from("")).await.unwrap();
    assert_eq!(res, Some(Response::Ok(String::from("PONG"))));
}

#[tokio::test]
async fn test_ping_with_value() {
    let addr = start_server().await.unwrap();
    let mut client = kvstore::client::create(addr).await.unwrap();
    let res = client.ping(String::from("Value")).await.unwrap();
    assert_eq!(res, Some(Response::Ok(String::from("Value"))));
}

/// Starts a server for integration tests.
/// This will start a server instance on a random non-used port.
async fn start_server() -> Result<SocketAddr> {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { kvstore::server::run(listener).await });
    Ok(addr)
}
