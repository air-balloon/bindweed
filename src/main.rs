pub fn should_have_default_header_paragraph_fields(file_line: &str) -> bool {
    let header_regex = Regex::new(r"^Format:.*$").unwrap();

    if !header_regex.is_match(file_line)
    {
        println!("The header must have at least a Format field.");
        return false;
    }

    true
}

pub fn should_have_header_paragraphl_format_ur(file_line: &str) -> bool {
    let header_regex = Regex::new(r"^Format: https?://\w+\.\w{1,63}(/\S*)?$").unwrap();
    if !header_regex.is_match(file_line)
    {
        println!("The format field should contain an URI");
        return false;
    }

    true
}
