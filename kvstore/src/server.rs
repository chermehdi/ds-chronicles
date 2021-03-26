use std::sync::{Arc, Mutex};

use crate::{
    executor::Executor,
    storage::{memory::InMemStorage, Storage},
    ConnectionHandler, Result,
};
use tokio::net::{TcpListener, TcpStream};

async fn process(
    stream: TcpStream,
    store: Arc<Mutex<Box<dyn Storage + Send + Sync>>>,
) -> Result<()> {
    let handler = ConnectionHandler::new(stream);
    let mut executor = Executor::new(handler, store);
    return executor.run().await;
}

pub async fn run(listener: TcpListener) {
    let store: Arc<Mutex<Box<dyn Storage + Send + Sync>>> =
        Arc::new(Mutex::new(Box::new(InMemStorage::new())));

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let store = Arc::clone(&store);
        tokio::spawn(async move {
            if let Err(msg) = process(stream, store).await {
                println!("An error happened while processing the request: {:?}", msg);
            }
        });
    }
}
