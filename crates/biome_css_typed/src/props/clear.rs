use biome_css_syntax::CssGenericComponentValueList;

use crate::diag::CssPropertyDiagnostic;
use crate::parse::ext::CssNodeExt;

use super::keyword::parse_keyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clear {
    None,
    Left,
    Right,
    Both,
    InlineStart,
    InlineEnd,
}

impl Clear {
    pub fn parse(value: &CssGenericComponentValueList) -> Result<Clear, CssPropertyDiagnostic> {
        parse_keyword(value, |ident| {
            if ident.ident_eq_ascii("none") {
                Some(Clear::None)
            } else if ident.ident_eq_ascii("left") {
                Some(Clear::Left)
            } else if ident.ident_eq_ascii("right") {
                Some(Clear::Right)
            } else if ident.ident_eq_ascii("both") {
                Some(Clear::Both)
            } else if ident.ident_eq_ascii("inline-start") {
                Some(Clear::InlineStart)
            } else if ident.ident_eq_ascii("inline-end") {
                Some(Clear::InlineEnd)
            } else {
                None
            }
        })
    }
}
