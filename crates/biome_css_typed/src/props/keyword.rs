use biome_css_syntax::{AnyCssGenericComponentValue, CssGenericComponentValueList};

use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};
use crate::parse::CssGenericComponentValueCursor;
use crate::parse::ext::CssNodeExt;

pub(super) fn parse_keyword<T, F>(
    list: &CssGenericComponentValueList,
    map: F,
) -> Result<T, CssPropertyDiagnostic>
where
    F: Fn(&AnyCssGenericComponentValue) -> Option<T>,
{
    let mut cursor = CssGenericComponentValueCursor::new(list);

    let first = cursor
        .bump_non_delim()
        .ok_or_else(|| CssPropertyDiagnostic {
            kind: CssPropertyDiagnosticKind::InvalidValue,
            range: None,
        })?;

    let is_identifier = first
        .as_any_css_value()
        .map(|value| {
            value
                .as_css_identifier()
                .and_then(|identifier| identifier.value_token().ok())
                .is_some()
                || value
                    .as_css_custom_identifier()
                    .and_then(|identifier| identifier.value_token().ok())
                    .is_some()
                || value
                    .as_css_dashed_identifier()
                    .and_then(|identifier| identifier.value_token().ok())
                    .is_some()
        })
        .unwrap_or(false);

    if !is_identifier {
        return Err(CssPropertyDiagnostic {
            kind: CssPropertyDiagnosticKind::InvalidValue,
            range: first.range_opt(),
        });
    }

    let keyword = map(&first).ok_or_else(|| CssPropertyDiagnostic {
        kind: CssPropertyDiagnosticKind::InvalidValue,
        range: first.range_opt(),
    })?;

    if let Some(extra) = cursor.peek_non_delim() {
        return Err(CssPropertyDiagnostic {
            kind: CssPropertyDiagnosticKind::UnexpectedToken,
            range: extra.range_opt(),
        });
    }

    Ok(keyword)
}
