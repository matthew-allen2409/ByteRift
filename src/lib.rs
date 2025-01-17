pub mod header;
pub mod request;
pub mod response;
pub mod router;
pub mod handler;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    OPTION,
    TRACE,
    PATCH
}

impl HttpMethod {
    pub fn from(str: &str) -> Result<Self, String> {
        match str {
            "GET" => Ok(HttpMethod::GET),
            "HEAD" => Ok(HttpMethod::HEAD),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "OPTION" => Ok(HttpMethod::OPTION),
            "TRACE" => Ok(HttpMethod::TRACE),
            "PATCH" => Ok(HttpMethod::PATCH),
            _ => Err(format!("Invalid method string: {str}")),
        }
    }
}
