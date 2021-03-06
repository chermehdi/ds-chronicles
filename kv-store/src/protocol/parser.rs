use crate::protocol::{get_string, get_u32, get_u8};
use crate::protocol::{Command, Response};
use crate::Result;
use std::io::Cursor;

/// Parser is a struct holder for protocol parsing methods.
pub struct Parser {}

impl Parser {
    /// Parse the given buffer into a `Command` instance.
    ///
    /// If the cursor contains data that does not adhere to the defined protocol, the function will
    /// return a `Result::Err`.
    ///     
    /// The contents of the cursor are not altered.  
    ///
    /// If the function was able to parse a prefix into a command, the cursor's pointer is advanced
    /// to the end of that prefix, yet failing in the middle of parsing the cursor can leave the
    /// internal pointer in any random location.
    pub fn parse(data: &mut Cursor<&[u8]>) -> Result<Command> {
        let _header = crate::protocol::get_u8(data)?;
        let command = get_u32(data)?;
        match command {
            0 => Parser::parse_get(data),
            1 => Parser::parse_set(data),
            2 => Parser::parse_clear(data),
            3 => Parser::parse_ping(data),
            _ => Err("Unknown command number".into()),
        }
    }

    /// Parse the given buffer into a `Response` instance.
    ///
    /// If the cursor contains data that does not adhere to the defined protocol, the function will
    /// return a `Result::Err`.
    ///     
    /// The contents of the cursor are not altered.  
    ///
    /// If the function was able to parse a prefix into a `Response`, the cursor's pointer is advanced
    /// to the end of that prefix, yet failing in the middle of parsing the cursor can leave the
    /// internal pointer in any random location.
    pub fn parse_response(data: &mut Cursor<&[u8]>) -> Result<Response> {
        let _header = get_u8(data)?;
        let response_type = get_u8(data)?;
        let response = match response_type {
            0 => Response::Ok(get_string(data)?),
            1 => Response::Error(get_string(data)?),
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
        Ok(Command::Clear(key))
    }

    fn parse_ping(data: &mut Cursor<&[u8]>) -> Result<Command> {
        let key = get_string(data)?;
        Ok(Command::Ping(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_get() {
        let mut buf: Vec<u8> = vec![];
        let command_num: u32 = 0;
        buf.push(0); // header bit
        write_u32(&mut buf, command_num);

        write_str(&mut buf, "foobar");

        let mut cur = Cursor::new(buf.as_slice());
        let command = Parser::parse(&mut cur).unwrap();

        match command {
            Command::Get(matched_key) => {
                assert_eq!(matched_key, "foobar");
            }
            // basically fail the test
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_works_for_clear() {
        let mut buf: Vec<u8> = vec![];
        let command_num: u32 = 2;
        buf.push(0); // header bit
        write_u32(&mut buf, command_num);
        write_str(&mut buf, "foobar");
        let mut cur = Cursor::new(buf.as_slice());
        let command = Parser::parse(&mut cur).unwrap();
        match command {
            Command::Clear(matched_key) => {
                assert_eq!(matched_key, "foobar");
            }
            // basically fail the test
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_works_for_set() {
        let mut buf: Vec<u8> = vec![];
        let command_num: u32 = 1;
        buf.push(0); // header bit
        write_u32(&mut buf, command_num);
        write_str(&mut buf, "foobar");
        write_str(&mut buf, "value");
        let mut cur = Cursor::new(buf.as_slice());
        let command = Parser::parse(&mut cur).unwrap();
        match command {
            Command::Set(matched_key, matched_value) => {
                assert_eq!(matched_key, "foobar");
                assert_eq!(matched_value, "value");
            }
            // basically fail the test
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_works_for_response_ok() {
        let mut buf: Vec<u8> = vec![];
        let response_type: u8 = 0; // ok
        buf.push(0); // header bit
        buf.push(response_type);
        write_str(&mut buf, "OK");
        let mut cur = Cursor::new(buf.as_slice());
        let response = Parser::parse_response(&mut cur).unwrap();
        match response {
            Response::Ok(matched_message) => {
                assert_eq!(matched_message, "OK");
            }
            // basically fail the test
            _ => assert_eq!(true, false),
        }
    }
    fn write_u32(buf: &mut Vec<u8>, val: u32) {
        buf.extend_from_slice(&val.to_be_bytes());
    }

    fn write_str<'a>(buf: &'a mut Vec<u8>, val: &'a str) {
        write_u32(buf, val.len() as u32);
        buf.extend_from_slice(val.as_bytes());
    }
}
