use biome_css_syntax::CssGenericComponentValueList;

use crate::diag::CssPropertyDiagnostic;
use crate::parse::ext::CssNodeExt;

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
            if ident.ident_eq_ascii("static") {
                Some(Position::Static)
            } else if ident.ident_eq_ascii("relative") {
                Some(Position::Relative)
            } else if ident.ident_eq_ascii("absolute") {
                Some(Position::Absolute)
            } else if ident.ident_eq_ascii("fixed") {
                Some(Position::Fixed)
            } else if ident.ident_eq_ascii("sticky") {
                Some(Position::Sticky)
            } else {
                None
            }
        })
    }
}
