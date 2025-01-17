use crate::response;
use crate::response::Response;
use crate::router::Router;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct Handler<T> {
    router: Router<T>,
    listener: TcpListener,
    config: T,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidHeaderError(String),
    InvalidRequestLine(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHeaderError(header) => write!(f, "Invalid Header: {}", header),
            Self::InvalidRequestLine(request_line) => write!(f, "Invalid Request Line: {}", request_line),
        }
    }
}


impl<T> Handler<T> {
    pub fn new(router: Router<T>, listener: TcpListener, config: T) -> Self {
        Handler {
            router,
            listener,
            config,
        }
    }

    pub fn listen(&self) {
        loop {
            self.listener.incoming().for_each(|stream| {
                thread::spawn(|| {
                    let stream = match stream {
                        Ok(stream) => Self::handle_request(stream),
                        Err(_) => return,
                    };
                });
            })
        }
    }

    fn handle_request(mut stream: TcpStream) {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut request_line = String::new();
        let mut headers = HashMap::new();
        match buf_reader.read_line(&mut request_line) {
            Ok(_) => (),
            Err(_) => {
                handle_error(stream, Error::InvalidRequestLine(request_line));
                return
            },
        };

        let mut header = String::new();
        loop {
            buf_reader.read_line(&mut header);

            if header.is_empty() {
                break;
            }
            let (key, value) = match parse_header(&header) {
                Ok((key, value)) => (key, value),
                Err(err) => {
                    handle_error(stream, err);
                    return
                }
            };
            headers.insert(key, value);
        }
    }
}

fn parse_header(header: &String) -> Result<(String, String), Error> {
    let mut parts = header.trim().splitn(2, ":");

    match (parts.next(), parts.next()) {
        (Some(key), Some(value)) => Ok((key.trim().to_string(), value.trim().to_string())),
        _ => Err(Error::InvalidHeaderError(header.to_string())),
    }
}

fn parse_headers(buf_reader: &mut BufReader<TcpStream>, headers: &mut HashMap<String, String>) -> Result<(), Error> {
    let mut header = String::new();
    loop {
        buf_reader.read_line(&mut header);

        if header.is_empty() {
            break;
        }
        let (key, value) = parse_header(&header)?;
        headers.insert(key, value);
    }

    Ok(())
}

fn handle_error(mut stream: TcpStream, error: Error) {
    match error {
        _ => {
            stream.write(
                &Response {
                    status_line: response::StatusLine::new(400, "Invalid request".to_string()),
                    headers: Vec::new(),
                    body: None
                }.into_bytes()
            );
        }
    }
}

#[cfg(test)]
mod tests;
