use biome_css_syntax::CssGenericComponentValueList;

use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};

use super::{
    box_sizing::BoxSizing, clear::Clear, float::Float, margin::MarginShorthand,
    padding::PaddingShorthand, position::Position, value::TypedPropertyValue,
    visibility::Visibility,
};

pub fn parse_property_typed(
    name: &str,
    list: &CssGenericComponentValueList,
) -> Result<TypedPropertyValue, CssPropertyDiagnostic> {
    let normalized = name.to_ascii_lowercase();

    match normalized.as_str() {
        "margin" => MarginShorthand::parse(list).map(TypedPropertyValue::Margin),
        "padding" => PaddingShorthand::parse(list).map(TypedPropertyValue::Padding),
        "visibility" => Visibility::parse(list).map(TypedPropertyValue::Visibility),
        "position" => Position::parse(list).map(TypedPropertyValue::Position),
        "float" => Float::parse(list).map(TypedPropertyValue::Float),
        "clear" => Clear::parse(list).map(TypedPropertyValue::Clear),
        "box-sizing" => BoxSizing::parse(list).map(TypedPropertyValue::BoxSizing),
        _ => Err(CssPropertyDiagnostic {
            kind: CssPropertyDiagnosticKind::UnknownProperty,
            range: None,
        }),
    }
}
