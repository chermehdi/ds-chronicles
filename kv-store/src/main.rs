use tokio::net::{TcpListener, TcpStream};

pub type Err = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Err>;

mod handler;
mod protocol;

async fn process(stream: TcpStream) -> Result<()> {
    let mut handler = handler::ConnectionHandler::new(stream);
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
                return Err(msg);
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let port = "6556";
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            if let Err(_msg) = process(stream).await {
                // log the error message
            }
        });
    }
}
