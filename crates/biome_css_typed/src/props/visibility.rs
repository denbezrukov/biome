use biome_css_syntax::CssGenericComponentValueList;

use crate::diag::CssPropertyDiagnostic;
use crate::parse::ext::CssNodeExt;

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
            if ident.ident_eq_ascii("visible") {
                Some(Visibility::Visible)
            } else if ident.ident_eq_ascii("hidden") {
                Some(Visibility::Hidden)
            } else if ident.ident_eq_ascii("collapse") {
                Some(Visibility::Collapse)
            } else {
                None
            }
        })
    }
}
