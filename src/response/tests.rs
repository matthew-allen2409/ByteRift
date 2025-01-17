use crate::response::StatusLine;

#[test]
fn status_line_new() {
    let status_text = String::from("foo bar");
    let expected = StatusLine {
        version: String::from("HTTP/1.1"),
        status_code: 200,
        status_text: status_text.clone(),
    };

    let result = StatusLine::new(200, status_text);

    assert_eq!(expected, result);
}

#[test]
fn to_string_expect() {
    let expected = String::from("HTTP/1.1 200 foo bar\r\n");

    let result = StatusLine::new(200, String::from("foo bar")).to_string();

    assert_eq!(expected, result);
}

