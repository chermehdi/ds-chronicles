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
async fn test_set_get() {
    let addr = start_server().await.unwrap();
    let mut client = kvstore::client::create(addr).await.unwrap();
    let res = client
        .set(String::from("key"), String::from("value"))
        .await
        .unwrap();
    assert_eq!(res, Some(Response::Ok(String::from("key"))));

    let res = client.get(String::from("key")).await.unwrap();
    assert_eq!(res, Some(Response::Ok(String::from("value"))));
}

#[tokio::test]
async fn test_set_override() {
    let addr = start_server().await.unwrap();
    let mut client = kvstore::client::create(addr).await.unwrap();
    let res = client
        .set(String::from("key"), String::from("value1"))
        .await
        .unwrap();
    assert_eq!(res, Some(Response::Ok(String::from("key"))));
    let res = client
        .set(String::from("key"), String::from("value2"))
        .await
        .unwrap();
    assert_eq!(res, Some(Response::Ok(String::from("key"))));

    let res = client.get(String::from("key")).await.unwrap();
    assert_eq!(res, Some(Response::Ok(String::from("value2"))));
}

#[tokio::test]
async fn test_unset() {
    let addr = start_server().await.unwrap();
    let mut client = kvstore::client::create(addr).await.unwrap();
    let res = client
        .set(String::from("key"), String::from("value"))
        .await
        .unwrap();
    assert_eq!(res, Some(Response::Ok(String::from("key"))));

    let res = client.unset(String::from("key")).await.unwrap();
    assert_eq!(res, Some(Response::Ok(String::from("value"))));

    // clients shouldn't be able to see the value after it's been deleted
    let res = client.get(String::from("key")).await.unwrap();
    assert_eq!(res, Some(Response::Error(String::from("Key not found"))));
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
