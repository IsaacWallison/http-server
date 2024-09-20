use super::method::{MethodError, Method};
use super::{QueryString, QueryStringValue};
use std::str::Utf8Error;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str;

#[derive(Debug)]
pub struct Request<'buffer> {
    path: &'buffer str,
    query_string: Option<QueryString<'buffer>>,
    method: Method
}

impl<'buffer> Request<'buffer> {
    pub fn ath(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}


impl<'buffer> TryFrom<&'buffer [u8]> for Request<'buffer> {
    type Error = ParseError;
     
    // GET /search?name=user&sort=1 HTTP/1.1\r\n HEADERS
    fn try_from(buffer: &'buffer [u8]) -> Result<Request<'buffer>, Self::Error> {
        let request = str::from_utf8(buffer)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    
    None
}


pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

// impl Request {
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
//         unimplemented!()
//     }
// }

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

//impl Error for ParseError {}