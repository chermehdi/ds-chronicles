use crate::Result;
use bytes::{Buf, BufMut};
use std::io::Cursor;
pub use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

mod parser;
pub use parser::Parser;

mod writer;
pub use writer::Writer;

#[derive(Debug)]
pub enum Command {
    Set(String, String),
    Get(String),
    Clear(String),
}

#[derive(Debug)]
pub enum Response {
    Ok(String),
    Error(String),
}

fn get_u8(cur: &mut Cursor<&[u8]>) -> Result<u8> {
    if !cur.has_remaining() {
        return Err("Buffer is exhausted".into());
    }
    Ok(cur.get_u8())
}

fn get_u32(cur: &mut Cursor<&[u8]>) -> Result<u32> {
    let line = get_slice(cur, 4)?;
    Ok(((line[0] as u32) << 24)
        + ((line[1] as u32) << 16)
        + ((line[2] as u32) << 8)
        + ((line[3] as u32) << 0))
}

fn get_string(cur: &mut Cursor<&[u8]>) -> Result<String> {
    let len = get_u32(cur)?;
    let data = get_slice(cur, len as usize)?;
    let key = String::from_utf8(data.to_vec())?;
    Ok(key)
}

fn get_slice<'a>(cur: &mut Cursor<&'a [u8]>, len: usize) -> Result<&'a [u8]> {
    let from = cur.position() as usize;
    let until = from + len;
    if cur.get_ref().len() < until {
        return Err("Buffer exhaused before being able to read the required data".into());
    }
    cur.set_position(until as u64);
    Ok(&cur.get_ref()[from..until])
}
