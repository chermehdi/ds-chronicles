use std::sync::{Arc, Mutex};

use crate::{
    executor::Executor,
    storage::{memory::InMemStorage, Storage},
    ConnectionHandler, Result, StorageOptions,
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

    // Calling open in this case is useless as the implementation does not
    // use open for anything, but it's just to showcase where you might want to
    // alter the code to use open in order to make it work with your own storage implementation.
    let _ = store.lock().unwrap().open(String::new(), StorageOptions {});

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
