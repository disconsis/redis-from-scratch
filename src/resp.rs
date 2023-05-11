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

/// read from the iterator till (and including) the next
/// CRLF, returning string preceding it
fn take_till_crlf<I>(bytes: &mut I) -> Option<&str>
    where I: Iterator<Item=u8>
{
    let content = str::from_utf8(
        & bytes
            .take_while(|b| *b != b'\r')
            .collect::<Vec<u8>>()
    ).ok();
    let end_ok = bytes.next() == Some(b'\n'); // got '\r\n', end of msg
    if ! end_ok { return None };
    content
}


impl Msg {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            SimpleString(s) => {
                vec![b"+", s.as_bytes(), b"\r\n"].concat()
            }

            _ => todo!("encode for {:?}", self)
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
                take_till_crlf(bytes)
                    .map(|s| SimpleString(s.to_string()))
            }

            // bulk string
            Some(b'$') => {
                todo!("decode for BulkString")
            }

            // error
            Some(b'-') => {
                todo!("decode for Error")
            }

            // integer
            Some(b':') => {
                todo!("decode for Integer")
            }

            // array
            Some(b'*') => {

            }

            _ => None
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
