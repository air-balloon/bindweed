use crate::ressources::Licence::Licence;

pub struct StandAloneParagraph {
    license: Licence,
    comment: Option<&'static str>,
}
