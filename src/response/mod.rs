use crate::header::Header;
use std::string::ToString;

#[derive(Debug, PartialEq)]
pub struct StatusLine {
    pub version: String,
    pub status_code: u16,
    pub status_text: String,
}

pub struct Response {
    pub status_line: StatusLine,
    pub headers: Vec<Header>,
    pub body: Option<Vec<u8>>,
}

impl StatusLine {
    pub fn new(status_code: u16, status_text: String) -> Self {
        Self {
            version: String::from("HTTP/1.1"),
            status_code,
            status_text,
        }
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl ToString for StatusLine {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}\r\n",
            self.version, self.status_code, self.status_text,
        )
    }
}

impl Response {
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.append(&mut self.status_line.into_bytes());

        bytes
    }
}

#[cfg(test)]
mod tests;
