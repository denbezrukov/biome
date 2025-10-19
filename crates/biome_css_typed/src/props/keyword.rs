use biome_css_syntax::CssGenericComponentValueList;
use biome_rowan::AstNode;

use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};
use crate::parse::CssGenericComponentValueCursor;

pub(super) fn parse_keyword<T, F>(
    list: &CssGenericComponentValueList,
    map: F,
) -> Result<T, CssPropertyDiagnostic>
where
    F: Fn(&str) -> Option<T>,
{
    let mut cursor = CssGenericComponentValueCursor::new(list);

    let first = cursor
        .bump_non_delim()
        .ok_or_else(|| CssPropertyDiagnostic {
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

    let ident = token.text_trimmed();

    let keyword = map(ident).ok_or_else(|| CssPropertyDiagnostic {
        kind: CssPropertyDiagnosticKind::InvalidValue,
        range: Some(first.range()),
    })?;

    if let Some(extra) = cursor.peek_non_delim() {
        return Err(CssPropertyDiagnostic {
            kind: CssPropertyDiagnosticKind::UnexpectedToken,
            range: Some(extra.range()),
        });
    }

    Ok(keyword)
}
