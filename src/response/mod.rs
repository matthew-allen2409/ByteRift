use crate::header::Header;

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

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}\r\n",
            self.version, self.status_code, self.status_text,
        )
    }
}

#[cfg(test)]
mod tests;
