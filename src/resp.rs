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
fn take_till_crlf<I>(bytes: &mut I) -> Option<String>
    where I: Iterator<Item=u8>
{
    let content = str::from_utf8(
        & bytes
            .take_while(|b| *b != b'\r')
            .collect::<Vec<u8>>()
    ).ok().map(String::from);
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
                take_till_crlf(bytes).map(SimpleString)
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
                let len = take_till_crlf(bytes)?.parse::<i8>().ok()?;
                if len < 0 { return Some(Null) };

                let mut parts: Vec<Msg> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    parts.push(Self::decode(bytes)?);
                }
                Some(Array(parts))
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
