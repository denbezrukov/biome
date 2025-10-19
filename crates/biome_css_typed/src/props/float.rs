use biome_css_syntax::CssGenericComponentValueList;

use crate::diag::CssPropertyDiagnostic;

use super::keyword::parse_keyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Float {
    Left,
    Right,
    None,
    InlineStart,
    InlineEnd,
}

impl Float {
    pub fn parse(value: &CssGenericComponentValueList) -> Result<Float, CssPropertyDiagnostic> {
        parse_keyword(value, |ident| {
            if ident.eq_ignore_ascii_case("left") {
                Some(Float::Left)
            } else if ident.eq_ignore_ascii_case("right") {
                Some(Float::Right)
            } else if ident.eq_ignore_ascii_case("none") {
                Some(Float::None)
            } else if ident.eq_ignore_ascii_case("inline-start") {
                Some(Float::InlineStart)
            } else if ident.eq_ignore_ascii_case("inline-end") {
                Some(Float::InlineEnd)
            } else {
                None
            }
        })
    }
}
