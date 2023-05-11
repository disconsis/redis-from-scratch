use std::str;

/// REdis Serialization Protocol (RESP) specification

#[derive(Debug)]
pub enum Msg {
    /// can only contain non-binary-safe string which
    /// don't contain a CR or LF character.
    SimpleString(String),
    /// can contain any binary-safe string.
    BulkString(String),
    /// error messages follow the same rules as `SimpleString`.
    Error(String),
    Integer(i64),
    /// heterogenous array of messages.
    /// nested arrays are possible.
    Array(Vec<Msg>),
    Null,
}

use Msg::*;

impl Msg {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            SimpleString(s) => {
                let mut enc = Vec::with_capacity(s.len() + 3);
                enc.push(b'+');
                enc.extend(s.bytes());
                enc.push(b'\r');
                enc.push(b'\n');
                enc
            }

            _ => todo!("decode for other types")
        }
    }

    /// TODO maybe return `Option<Result<Msg>>` in case of decoding errors?
    pub fn decode<I>(bytes: &mut I) -> Option<Self>
        where I: Iterator<Item=u8>
    {
        let first = bytes.next();
        match first {
            // simple string
            Some(b'+') => {
                let content = str::from_utf8(
                    & bytes
                        .take_while(|b| *b != b'\r')
                        .collect::<Vec<u8>>()
                ).ok()?.to_string();
                let end_ok = bytes.next() == Some(b'\n'); // got '\r\n', end of msg
                if end_ok {
                    Some(SimpleString(content))
                } else {
                    None
                }
           }

            _ => todo!("decode for other types")
        }
    }

    pub fn decoder<I>(bytes: I) -> Decoder<I>
        where I: Iterator<Item=u8>
    {
        Decoder(bytes)
    }
}


/// decoder over an iterator, which returns an iterator of the decoded type
pub struct Decoder<I>(I);

impl<I> Iterator for Decoder<I> where I: Iterator<Item=u8>
{
    type Item = Msg;

    fn next(&mut self) -> Option<Self::Item> {
        Msg::decode(&mut self.0)
    }
}
