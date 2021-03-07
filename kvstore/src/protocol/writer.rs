use crate::protocol::Command;
use crate::protocol::Response;
use std::io::Result;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

pub struct Writer {}

impl Writer {
    /// Write the given command to the output stream asynchronously.
    ///
    /// Any errors occured at the moment of writing to the output stream are propagated back to the
    /// caller.
    pub async fn write_command(buf: &mut BufWriter<TcpStream>, cmd: &Command) -> Result<()> {
        match cmd {
            Command::Get(key) => {
                buf.write_u8(0).await?;
                buf.write_u16(0).await?;
                write_string(buf, &key).await?;
            }
            Command::Set(key, value) => {
                buf.write_u8(0).await?;
                buf.write_u16(1).await?;
                write_string(buf, &key).await?;
                write_string(buf, &value).await?;
            }
            Command::Clear(key) => {
                buf.write_u8(0).await?;
                buf.write_u16(2).await?;
                write_string(buf, &key).await?;
            }
            Command::Ping(key) => {
                buf.write_u8(0).await?;
                buf.write_u16(3).await?;
                if key.is_empty() {
                    // Default to `PONG`
                    write_string(buf, &"PONG".into()).await?;
                } else {
                    write_string(buf, &key).await?;
                }
            }
        }

        // ensure the buffered stream is flushed into the socket.
        // if we don't flush explicitly, no data will be written to the socket until
        // the buffer is full.
        buf.flush().await?;

        Ok(())
    }

    /// Write the given Response to the output stream asynchronously.
    ///
    /// Any errors occured at the moment of writing to the output stream are propagated back to the
    /// caller.
    pub async fn write_response(buf: &mut BufWriter<TcpStream>, res: &Response) -> Result<()> {
        match res {
            Response::Ok(msg) => {
                buf.write_u8(0).await?;
                // 0 indicates success status
                buf.write_u8(0).await?;
                write_string(buf, &msg).await?;
            }
            Response::Error(msg) => {
                buf.write_u8(0).await?;
                // 1 indicates failure status
                buf.write_u8(1).await?;
                write_string(buf, &msg).await?;
            }
        }
        buf.flush().await?;
        Ok(())
    }
}

// Utility method to write a string to an output stream in a standard format, 4 bytes for the
// length `n`, followd by `n` bytes of the actual string.
async fn write_string(buf: &mut BufWriter<TcpStream>, data: &String) -> Result<()> {
    let len = data.len() as u32;
    buf.write_u32(len).await?;
    buf.write(data.as_bytes()).await?;
    Ok(())
}
