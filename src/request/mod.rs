use std::str::FromStr;
use std::collections::HashMap;
use crate::HttpMethod;

#[derive(Debug, PartialEq)]
pub struct RequestLine {
    pub method: HttpMethod,
    pub target: String,
    pub version: String,
}

pub struct Request {
    pub request_line: RequestLine,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl FromStr for RequestLine {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, String> {
        let mut parts = str.split_whitespace();
        let method = match parts.next() {
            Some(method) => method,
            None => return Err(format!("Invalid request line string: {str}")),
        };
        let method = HttpMethod::from(method)?;
        let target = match parts.next() {
            Some(target) => target,
            None => return Err(format!("Invalid request line string: {str}")),
        }.to_string();
        let version = match parts.next() {
            Some(version) => version,
            None => return Err(format!("Invalid request line string: {str}")),
        }.to_string();

        Ok(RequestLine {method, target, version})
    }
}

#[cfg(test)]
mod tests;
