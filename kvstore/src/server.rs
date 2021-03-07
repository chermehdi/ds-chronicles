use crate::{ConnectionHandler, Result};
use tokio::net::{TcpListener, TcpStream};

async fn process(stream: TcpStream) -> Result<()> {
    let mut handler = ConnectionHandler::new(stream);
    loop {
        match handler.read_command().await {
            Ok(val) => match val {
                Some(cmd) => {
                    handler.execute(cmd).await?;
                }
                None => {
                    break;
                }
            },
            Err(msg) => {
                println!("matched an error {:?}", msg);
                return Err(msg);
            }
        }
    }
    Ok(())
}

pub async fn run(listener: TcpListener) {
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            if let Err(msg) = process(stream).await {
                println!("An error happened while processing the request: {:?}", msg);
            }
        });
    }
}
