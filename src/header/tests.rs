use crate::header::Header;

#[test]
fn new_expect_correct() {
    let name = String::from("foo");
    let value = String::from("bar");
    let expected = Header {
        name: name.clone(),
        value: value.clone(),
    };

    let result = Header::new(name, value).unwrap();

    assert_eq!(expected, result);
}

#[test]
fn new_empty_name_err() {
    let name = String::from("");
    let value = String::from("bar");

    let result = Header::new(name, value);

    assert!(result.is_err());
}

#[test]
fn new_empty_value_err() {
    let name = String::from("foo");
    let value = String::from("");

    let result = Header::new(name, value);

    assert!(result.is_err());
}

#[test]
fn new_empty_err() {
    let name = String::from("");
    let value = String::from("");

    let result = Header::new(name, value);

    assert!(result.is_err());
}

#[test]
fn to_string_expect() {
    let name = String::from("foo");
    let value = String::from("bar");
    let expected = String::from("foo: bar\r\n");

    let result = Header::new(name, value).unwrap().to_string();

    assert_eq!(expected, result);
}
