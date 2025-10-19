use biome_css_syntax::CssGenericComponentValueList;
use biome_rowan::AstNode;
use biome_string_case::StrLikeExtension;

use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};
use crate::parse::CssGenericComponentValueCursor;

use super::value::PropertyValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

impl Visibility {
    pub fn parse(
        value: &CssGenericComponentValueList,
    ) -> Result<PropertyValue, CssPropertyDiagnostic> {
        let mut cur = CssGenericComponentValueCursor::new(value);

        let first = cur.bump_non_delim().ok_or_else(|| CssPropertyDiagnostic {
            kind: CssPropertyDiagnosticKind::InvalidValue,
            range: None,
        })?;

        let token = first
            .as_any_css_value()
            .and_then(|value| value.as_css_identifier())
            .and_then(|identifier| identifier.value_token().ok())
            .ok_or_else(|| CssPropertyDiagnostic {
                kind: CssPropertyDiagnosticKind::InvalidValue,
                range: Some(first.range()),
            })?;
        let ident = token.text_trimmed().to_ascii_lowercase_cow();

        let vis = match ident.as_ref() {
            "visible" => Visibility::Visible,
            "hidden" => Visibility::Hidden,
            "collapse" => Visibility::Collapse,
            _ => {
                return Err(CssPropertyDiagnostic {
                    kind: CssPropertyDiagnosticKind::InvalidValue,
                    range: Some(first.range()),
                });
            }
        };

        if let Some(extra) = cur.peek_non_delim() {
            return Err(CssPropertyDiagnostic {
                kind: CssPropertyDiagnosticKind::UnexpectedToken,
                range: Some(extra.range()),
            });
        }

        Ok(PropertyValue::Visibility(vis))
    }
}

