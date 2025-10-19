use biome_css_syntax::{AnyCssDimension, AnyCssGenericComponentValue, AnyCssValue};

use super::dimension::{Length, LengthUnit};
use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};
use crate::parse::CssGenericComponentValueCursor;
use crate::parse::ext::CssNodeExt;

pub fn parse_length(
    cur: &mut CssGenericComponentValueCursor,
) -> Result<Length, CssPropertyDiagnostic> {
    cur.skip_delims();
    let node = cur.bump().ok_or_else(|| invalid_value(None))?;
    let range = node.range_opt();

    let Some(AnyCssValue::AnyCssDimension(dimension)) = node.as_any_css_value() else {
        return Err(invalid_value(range));
    };

    if matches!(dimension, AnyCssDimension::CssPercentage(_)) {
        return Err(invalid_value(range));
    }

    let Some(unit) = length_unit_from_node(&node) else {
        return Err(invalid_value(range));
    };

    let Some(value) = dimension_value(&dimension) else {
        return Err(invalid_value(range));
    };

    Ok(Length { value, unit })
}

pub(crate) fn dimension_value(dimension: &AnyCssDimension) -> Option<f64> {
    let token = match dimension {
        AnyCssDimension::CssPercentage(percent) => percent.value_token().ok()?,
        AnyCssDimension::CssRegularDimension(dimension) => dimension.value_token().ok()?,
        AnyCssDimension::CssUnknownDimension(dimension) => dimension.value_token().ok()?,
    };

    token.text_trimmed().parse::<f64>().ok()
}

fn length_unit_from_node(node: &AnyCssGenericComponentValue) -> Option<LengthUnit> {
    let Some(AnyCssValue::AnyCssDimension(dimension)) = node.as_any_css_value() else {
        return None;
    };

    let token = match dimension {
        AnyCssDimension::CssPercentage(_) => return None,
        AnyCssDimension::CssRegularDimension(dimension) => dimension.unit_token().ok()?,
        AnyCssDimension::CssUnknownDimension(dimension) => dimension.unit_token().ok()?,
    };

    LengthUnit::from_str(token.text_trimmed())
}

fn invalid_value(range: Option<biome_text_size::TextRange>) -> CssPropertyDiagnostic {
    CssPropertyDiagnostic {
        kind: CssPropertyDiagnosticKind::InvalidValue,
        range,
    }
}
