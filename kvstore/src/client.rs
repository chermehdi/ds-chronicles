use crate::handler::ConnectionHandler;
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::protocol::{Command, Response};
use crate::Result;

pub struct Client {
    handler: ConnectionHandler,
}

pub async fn create<T: ToSocketAddrs>(addr: T) -> Result<Client> {
    let stream = TcpStream::connect(addr).await?;
    let handler = ConnectionHandler::new(stream);
    Ok(Client { handler })
}

impl Client {
    pub async fn set(&mut self, key: String, value: String) -> Result<Option<Response>> {
        let command = Command::Set(key, value);
        self.handler.write_command(&command).await?;
        return self.handler.read_response().await;
    }

    pub async fn get(&mut self, key: String) -> Result<Option<Response>> {
        let command = Command::Get(key);
        self.handler.write_command(&command).await?;
        return self.handler.read_response().await;
    }

    pub async fn unset(&mut self, key: String) -> Result<Option<Response>> {
        let command = Command::Clear(key);
        self.handler.write_command(&command).await?;
        return self.handler.read_response().await;
    }

    pub async fn ping(&mut self, key: String) -> Result<Option<Response>> {
        let command = Command::Ping(key);
        self.handler.write_command(&command).await?;
        return self.handler.read_response().await;
    }
}
