use crate::protocol::{get_string, get_u32, get_u8};
use crate::protocol::{Command, Response};
use crate::Result;
use std::io::Cursor;

pub struct Parser {}

impl Parser {
    pub fn parse(data: &mut Cursor<&[u8]>) -> Result<Command> {
        let _header = crate::protocol::get_u8(data)?;
        let command = get_u32(data)?;
        match command {
            0 => Parser::parse_get(data),
            1 => Parser::parse_set(data),
            2 => Parser::parse_clear(data),
            _ => Err("Unknown command number".into()),
        }
    }

    pub fn parse_response(data: &mut Cursor<&[u8]>) -> Result<Response> {
        let _header = get_u8(data)?;
        let response_type = get_u8(data)?;
        let response = match response_type {
            b'0' => Response::Ok(get_string(data)?),
            b'1' => Response::Error(get_string(data)?),
            _ => Response::Error("Unknown response type".into()),
        };

        Ok(response)
    }

    fn parse_get(data: &mut Cursor<&[u8]>) -> Result<Command> {
        let key = get_string(data)?;
        Ok(Command::Get(key))
    }

    fn parse_set(data: &mut Cursor<&[u8]>) -> Result<Command> {
        let key = get_string(data)?;
        let value = get_string(data)?;
        Ok(Command::Set(key, value))
    }

    fn parse_clear(data: &mut Cursor<&[u8]>) -> Result<Command> {
        let key = get_string(data)?;
        Ok(Command::Get(key))
    }
}
