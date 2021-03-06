use bytes::{Buf, BytesMut};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, BufWriter};
use tokio::net::TcpStream;

use crate::protocol;
use crate::Result;

/// A struct to encapsulate read / write logic of between the client and server.
///
/// There is a 1:1 relationship between the client and a handler, it is used to hide the
/// networking logic from other components.
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
            buf: BytesMut::with_capacity(1 << 16),
        }
    }

    pub async fn read_command(&mut self) -> Result<Option<protocol::Command>> {
        if self.buf.is_empty() {
            if self.stream.read_buf(&mut self.buf).await? == 0 {
                // Connection is closed from the client
                return Ok(None);
            }
        }
        let mut cursor = Cursor::new(&self.buf[..]);
        let command = protocol::Parser::parse(&mut cursor)?;
        // If reading the command was successfull, we should advance the internal
        // pointer of the buffer as the cursor only updates it's own.
        let length = cursor.position() as usize;
        self.buf.advance(length);

        Ok(Some(command))
    }

    pub async fn execute(&mut self, cmd: protocol::Command) -> Result<()> {
        match cmd {
            protocol::Command::Ping(key) => {
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
            protocol::Writer::write_response(
                &mut self.stream,
                &protocol::Response::Ok("PONG".into()),
            )
            .await?;
        } else {
            protocol::Writer::write_response(&mut self.stream, &protocol::Response::Ok(key))
                .await?;
        }
        Ok(())
    }
}
