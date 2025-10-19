use biome_css_syntax::{AnyCssDimension, AnyCssValue};

use super::parse_length::dimension_value;
use super::percent::Percentage;
use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};
use crate::parse::CssGenericComponentValueCursor;
use crate::parse::ext::CssNodeExt;

pub fn parse_percentage(
    cur: &mut CssGenericComponentValueCursor,
) -> Result<Percentage, CssPropertyDiagnostic> {
    cur.skip_delims();
    let node = cur.bump().ok_or_else(|| invalid_value(None))?;
    let range = node.range_opt();

    let Some(AnyCssValue::AnyCssDimension(dimension)) = node.as_any_css_value() else {
        return Err(invalid_value(range));
    };

    if !node.unit_eq_ascii("%") {
        return Err(invalid_value(range));
    }

    if !matches!(dimension, AnyCssDimension::CssPercentage(_)) {
        return Err(invalid_value(range));
    }

    let Some(value) = dimension_value(&dimension) else {
        return Err(invalid_value(range));
    };

    Ok(Percentage(value))
}

fn invalid_value(range: Option<biome_text_size::TextRange>) -> CssPropertyDiagnostic {
    CssPropertyDiagnostic {
        kind: CssPropertyDiagnosticKind::InvalidValue,
        range,
    }
}
