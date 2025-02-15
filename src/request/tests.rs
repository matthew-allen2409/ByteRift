use crate::request::RequestLine;
use crate::HttpMethod;

#[test]
fn request_line_from_string_expect() {
    let request_line_string = "GET /foo/bar HTTP/1.1";
    let expected = RequestLine {
        method: HttpMethod::GET,
        target: String::from("/foo/bar"),
        version: String::from("HTTP/1.1"),
    };

    let result = request_line_string.parse().unwrap();

    assert_eq!(expected, result);
}

#[test]
fn request_line_from_string_empty_expect_err() {
    let request_line_string = "";

    let result: Result<RequestLine, _> = request_line_string.parse();

    assert!(result.is_err());
}

#[test]
fn request_line_from_string_empty_method_expect_err() {
    let request_line_string = "/foo/bar HTTP/1.1";

    let result: Result<RequestLine, _> = request_line_string.parse();

    assert!(result.is_err());
}

#[test]
fn request_line_from_string_empty_target_expect_err() {
    let request_line_string = "GET HTTP/1.1";

    let result: Result<RequestLine, _> = request_line_string.parse();

    assert!(result.is_err());
}

#[test]
fn request_line_from_string_empty_version_expect_err() {
    let request_line_string = "GET /foo/bar";

    let result: Result<RequestLine, _> = request_line_string.parse();

    assert!(result.is_err());
}
