use crate::ressources::Copyright::Copyright;

pub struct FileParagraph {
    files: &'static str,
    copyright: Vec<Copyright>,
    license: &'static str,
    comment: Option<&'static str>,
}

pub struct Pattern {
    path: &'static str,
    is_pattern: bool,
    is_file: bool
}
