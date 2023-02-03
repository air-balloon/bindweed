use crate::ressources::Licence::Licence;

pub struct HeaderParagraph {
    format: &'static str,
    upsteam_name: Option<&'static str>,
    upstream_contact: Option<&'static str>,
    source: Option<&'static str>,
    disclaimer: Option<&'static str>,
    comment: Option<&'static str>,
    license: Option<Licence>,
    copyright: Option<&'static str>,
}

pub const STANDARD_HTTP_URL: &str = "http://www.debian.org/doc/packaging-manuals/copyright-format/1.0/";
pub const STANDARD_HTTPS_URL: &str = "https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/";
pub const OLD_HTTP_URL: &str = "http://anonscm.debian.org/viewvc/dep/web/deps/dep5.mdwn";
