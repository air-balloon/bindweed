#[cfg(test)]
use super::*;

#[test]
fn it_shouldnt_begin_with_something_else_on_first_line() {
    let listOfLine: &str = " Format";

    assert_eq!(
        false,
        should_have_default_header_paragraph_fields(listOfLine)
    )
}

#[test]
fn it_should_contain_an_uri() {
    let line: &str = "Format: aazeaezazea";

    assert_eq!(
        false,
        should_have_header_paragraphl_format_ur(line)
    )
}

#[test]
fn it_should_contain_a_valid_uri() {
    let line: &str = "Format: http://";

    assert_eq!(
        false,
        should_have_header_paragraphl_format_ur(line)
    )
}

#[test]
fn it_should_contain_a_valid_http_uri() {
    let line: &str = "Format: http://exempl.xyz";

    assert_eq!(
        true,
        should_have_header_paragraphl_format_ur(line)
    )
}

#[test]
fn it_should_contain_a_valid_https_uri() {
    let line: &str = "Format: https://exempl.xyz";

    assert_eq!(
        true,
        should_have_header_paragraphl_format_ur(line)
    )
}

#[test]
fn it_should_contain_a_valid_uri_with_a_tld() {
    let line: &str = "Format: https://exempl";

    assert_eq!(
        false,
        should_have_header_paragraphl_format_ur(line)
    )
}

#[test]
fn it_should_contain_a_valid_uri_with_a_domain_name() {
    let line: &str = "Format: https://.tld";

    assert_eq!(
        false,
        should_have_header_paragraphl_format_ur(line)
    )
}

#[test]
fn it_should_begin_with_format_on_first_line() {
    let listOfLine: &str = "Format:";

    assert_eq!(
        true,
        should_have_default_header_paragraph_fields(listOfLine)
    )
}
