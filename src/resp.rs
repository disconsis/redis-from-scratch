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
use anyhow::bail;

/// read from the iterator till (and including) the next
/// CRLF, returning string preceding it
fn take_till_crlf<I>(bytes: &mut I) -> anyhow::Result<String>
    where I: Iterator<Item=u8>
{
    let content = str::from_utf8(
        & bytes
            .take_while(|b| *b != b'\r')
            .collect::<Vec<u8>>()
    ).map(ToString::to_string);
    let end_ok = bytes.next() == Some(b'\n'); // got '\r\n', end of msg
    if ! end_ok { bail!("didn't receive a '\\r\\n' at the end of the msg part") };
    content.map_err(Into::into)
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

    pub fn decode<I>(bytes: &mut I) -> Option<anyhow::Result<Self>>
        where I: Iterator<Item=u8>
    {
        Some(Self::decode_with_first_byte(bytes.next()?, bytes))
    }

    fn decode_with_first_byte<I>(first_byte: u8, bytes: &mut I) -> anyhow::Result<Self>
        where I: Iterator<Item=u8>
    {
        match first_byte {
            // simple string
            b'+' => {
                take_till_crlf(bytes).map(SimpleString)
            }

            // bulk string
            b'$' => {
                let len = take_till_crlf(bytes)
                    .and_then(|s| s.parse::<i8>().map_err(Into::into))?;
                if len < 0 { return Ok(Null) };

                let result = str::from_utf8(
                    & bytes
                        .take(len as usize)
                        .collect::<Vec<u8>>()
                ).map(|s| BulkString(String::from(s)))
                 .map_err(Into::into);

                let end_bytes = bytes.take(2).collect::<Vec<u8>>();
                if end_bytes != b"\r\n" {
                    bail!("erroneous end bytes {:?} instead of '\\r\\n'", end_bytes)
                }
                result
            }

            // error
            b'-' => {
                todo!("decode for Error")
            }

            // integer
            b':' => {
                todo!("decode for Integer")
            }

            // array
            b'*' => {
                let len = take_till_crlf(bytes)?.parse::<i8>()?;
                if len < 0 { return Ok(Null) };

                let mut parts: Vec<Msg> = Vec::with_capacity(len as usize);
                for i in 0..len {
                    match Self::decode(bytes) {
                        Some(Ok(part)) => { parts.push(part); }
                        Some(Err(err)) => { return Err(err.context(format!("reading array index {i} of {len}"))); }
                        None => { bail!("EOF while reading array index {} of {}", i, len) }
                    }
                }
                Ok(Array(parts))
            }

            _ => bail!("unexpected first byte {:?} in RESP encoding", first_byte as char)
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
    type Item = anyhow::Result<Msg>;

    fn next(&mut self) -> Option<Self::Item> {
        Msg::decode(&mut self.0)
    }
}
