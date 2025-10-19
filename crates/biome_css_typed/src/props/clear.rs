use biome_css_syntax::CssGenericComponentValueList;

use crate::diag::CssPropertyDiagnostic;

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
            if ident.eq_ignore_ascii_case("none") {
                Some(Clear::None)
            } else if ident.eq_ignore_ascii_case("left") {
                Some(Clear::Left)
            } else if ident.eq_ignore_ascii_case("right") {
                Some(Clear::Right)
            } else if ident.eq_ignore_ascii_case("both") {
                Some(Clear::Both)
            } else if ident.eq_ignore_ascii_case("inline-start") {
                Some(Clear::InlineStart)
            } else if ident.eq_ignore_ascii_case("inline-end") {
                Some(Clear::InlineEnd)
            } else {
                None
            }
        })
    }
}
