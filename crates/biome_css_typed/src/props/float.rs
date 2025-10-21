use biome_css_syntax::CssGenericComponentValueList;

use crate::diag::CssPropertyDiagnostic;
use crate::parse::ext::CssNodeExt;

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
            if ident.ident_eq_ascii("left") {
                Some(Float::Left)
            } else if ident.ident_eq_ascii("right") {
                Some(Float::Right)
            } else if ident.ident_eq_ascii("none") {
                Some(Float::None)
            } else if ident.ident_eq_ascii("inline-start") {
                Some(Float::InlineStart)
            } else if ident.ident_eq_ascii("inline-end") {
                Some(Float::InlineEnd)
            } else {
                None
            }
        })
    }
}
