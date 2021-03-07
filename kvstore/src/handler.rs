use bytes::{Buf, BytesMut};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, BufWriter};
use tokio::net::TcpStream;

use crate::protocol::{Command, Parser, Response, Writer};
use crate::Result;

/// A struct to encapsulate read / write logic of between the client and server.
///
/// There is a 1:1 relationship between the client and a handler, it is used to hide the
/// networking logic from other components.
#[derive(Debug)]
pub struct ConnectionHandler {
    // created per client connection, used to read/write commands and responses.
    stream: BufWriter<TcpStream>,

    // used as a temporary buffer of the data sent from the client.
    buf: BytesMut,
}

impl ConnectionHandler {
    pub fn new(stream: TcpStream) -> Self {
        ConnectionHandler {
            stream: BufWriter::new(stream),
            // size of the buffer is kind of arbitrary here.
            buf: BytesMut::with_capacity(1 << 10),
        }
    }

    pub async fn read_command(&mut self) -> Result<Option<Command>> {
        if let None = self.ensure_filled().await? {
            // `None` is returned when the connection is closed and no bytes can be read.
            return Ok(None);
        }

        let mut cursor = Cursor::new(&self.buf[..]);
        let command = Parser::parse(&mut cursor)?;
        // If reading the command was successfull, we should advance the internal
        // pointer of the buffer as the cursor only updates it's own.
        let length = cursor.position() as usize;
        self.buf.advance(length);

        Ok(Some(command))
    }

    pub async fn write_response(&mut self, resp: &Response) -> Result<()> {
        Writer::write_response(&mut self.stream, resp).await?;
        Ok(())
    }

    pub async fn write_command(&mut self, cmd: &Command) -> Result<()> {
        Writer::write_command(&mut self.stream, cmd).await?;
        Ok(())
    }

    pub async fn read_response(&mut self) -> Result<Option<Response>> {
        if let None = self.ensure_filled().await? {
            // `None` is returned when the connection is closed and no bytes can be read.
            return Ok(None);
        }
        let mut cursor = Cursor::new(&self.buf[..]);
        let response = Parser::parse_response(&mut cursor)?;
        // If reading the command was successfull, we should advance the internal
        // pointer of the buffer as the cursor only updates it's own.
        let length = cursor.position() as usize;
        self.buf.advance(length);

        Ok(Some(response))
    }

    async fn ensure_filled(&mut self) -> Result<Option<usize>> {
        // TODO: clean this up. Use a proper enum to indicate the state of the buffer.
        println!("Trying to fill the buffer {:}", self.buf.len());
        if self.buf.is_empty() {
            let read = match self.stream.read_buf(&mut self.buf).await {
                Ok(read) => {
                    println!("Something happened {:?}", read);
                    read
                }
                Err(msg) => {
                    println!("Something happened {:?}", msg);
                    0
                }
            };
            if read == 0 {
                // Connection is closed from the client
                return Ok(None);
            }
            return Ok(Some(read));
        }
        Ok(Some(0))
    }

    pub async fn execute(&mut self, cmd: Command) -> Result<()> {
        match cmd {
            Command::Ping(key) => {
                return self.handle_ping(key).await;
            }
            _ => {
                unimplemented!();
            }
        }
    }

    async fn handle_ping(&mut self, key: String) -> Result<()> {
        if key.is_empty() {
            // Default to a ping.
            self.write_response(&Response::Ok("PONG".into())).await?;
        } else {
            self.write_response(&Response::Ok(key)).await?;
        }
        Ok(())
    }
}
