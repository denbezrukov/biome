use biome_css_syntax::CssGenericComponentValueList;

use crate::diag::CssPropertyDiagnostic;

use super::keyword::parse_keyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Position {
    pub fn parse(value: &CssGenericComponentValueList) -> Result<Position, CssPropertyDiagnostic> {
        parse_keyword(value, |ident| {
            if ident.eq_ignore_ascii_case("static") {
                Some(Position::Static)
            } else if ident.eq_ignore_ascii_case("relative") {
                Some(Position::Relative)
            } else if ident.eq_ignore_ascii_case("absolute") {
                Some(Position::Absolute)
            } else if ident.eq_ignore_ascii_case("fixed") {
                Some(Position::Fixed)
            } else if ident.eq_ignore_ascii_case("sticky") {
                Some(Position::Sticky)
            } else {
                None
            }
        })
    }
}
