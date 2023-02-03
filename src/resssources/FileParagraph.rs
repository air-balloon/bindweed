pub struct FileParagraph {
    files: &str,
    copyright: Vec<Copyright>,
    license: &str,
    comment: Option<&str>,
}

pub struct Pattern {
    path: &str,
    is_pattern: bool,
    is_file: bool
}
