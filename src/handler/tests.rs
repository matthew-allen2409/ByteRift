#[test]
pub fn parse_header_valid_header() {
    let header = String::from("key: value");
    let expected = (String::from("key"), String::from("value"));

    let result = super::parse_header(&header).unwrap();

    assert_eq!(expected, result);
}

#[test]
pub fn parse_header_missing_colon_header() {
    let header = String::from("key value");
    let expected = Err(super::Error::InvalidHeaderError(header.clone()));

    let result = super::parse_header(&header);

    assert_eq!(expected, result);
}

#[test]
pub fn parse_header_multiple_colon_header() {
    let header = String::from("key: value :: namespace");
    let expected = Ok(("key".to_string(), "value :: namespace".to_string()));

    let result = super::parse_header(&header);

    assert_eq!(expected, result);
}


#[test]
pub fn parse_header_padding() {
    let header = String::from("     key:    value      ");
    let expected = Ok((String::from("key"), String::from("value")));

    let result = super::parse_header(&header);

    assert_eq!(expected, result);
}
