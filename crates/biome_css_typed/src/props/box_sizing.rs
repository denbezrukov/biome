use biome_css_syntax::CssGenericComponentValueList;

use crate::diag::CssPropertyDiagnostic;

use super::keyword::parse_keyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

impl BoxSizing {
    pub fn parse(value: &CssGenericComponentValueList) -> Result<BoxSizing, CssPropertyDiagnostic> {
        parse_keyword(value, |ident| {
            if ident.eq_ignore_ascii_case("content-box") {
                Some(BoxSizing::ContentBox)
            } else if ident.eq_ignore_ascii_case("border-box") {
                Some(BoxSizing::BorderBox)
            } else {
                None
            }
        })
    }
}
