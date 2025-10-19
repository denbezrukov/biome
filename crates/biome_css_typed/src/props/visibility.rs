use crate::diag::CssPropertyDiagnostic;
use biome_css_syntax::CssGenericComponentValueList;

use super::keyword::parse_keyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

impl Visibility {
    pub fn parse(
        value: &CssGenericComponentValueList,
    ) -> Result<Visibility, CssPropertyDiagnostic> {
        parse_keyword(value, |ident| {
            if ident.eq_ignore_ascii_case("visible") {
                Some(Visibility::Visible)
            } else if ident.eq_ignore_ascii_case("hidden") {
                Some(Visibility::Hidden)
            } else if ident.eq_ignore_ascii_case("collapse") {
                Some(Visibility::Collapse)
            } else {
                None
            }
        })
    }
}
