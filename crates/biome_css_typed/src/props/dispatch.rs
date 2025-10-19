use biome_css_syntax::CssGenericComponentValueList;

use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};

use super::{value::PropertyValue, visibility::Visibility};

pub fn parse_property(
    name: &str,
    value: &CssGenericComponentValueList,
) -> Result<PropertyValue, CssPropertyDiagnostic> {
    match name {
        "visibility" => Visibility::parse(value),
        _ => Err(CssPropertyDiagnostic {
            kind: CssPropertyDiagnosticKind::UnknownProperty,
            range: None,
        }),
    }
}
