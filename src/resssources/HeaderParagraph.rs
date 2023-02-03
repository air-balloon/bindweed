pub struct HeaderParagraph {
    format: &str,
    upsteam_name: Option<&str>,
    upstream_contact: Option<&str>,
    source: Option<&str>,
    disclaimer: Option<&str>,
    comment: Option<&str>,
    license: Option<Licence>,
    copyright: Option<&str>,
}
