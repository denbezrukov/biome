use biome_css_syntax::{AnyCssDeclarationName, CssGenericProperty};

use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};

use super::{padding::PaddingShorthand, value::PropertyValue, visibility::Visibility};

pub fn parse_property(
    property: &CssGenericProperty,
) -> Result<PropertyValue, CssPropertyDiagnostic> {
    let value = property.value();
    let Some(name_token) = property.name().ok().and_then(|name| match name {
        AnyCssDeclarationName::CssIdentifier(identifier) => identifier.value_token().ok(),
        AnyCssDeclarationName::CssDashedIdentifier(identifier) => identifier.value_token().ok(),
    }) else {
        return Err(CssPropertyDiagnostic {
            kind: CssPropertyDiagnosticKind::UnknownProperty,
            range: None,
        });
    };

    let range = name_token.text_trimmed_range();
    let name = name_token.text_trimmed();

    if name.eq_ignore_ascii_case("visibility") {
        Visibility::parse(&value)
    } else if name.eq_ignore_ascii_case("padding") {
        PaddingShorthand::parse(&value)
    } else {
        Err(CssPropertyDiagnostic {
            kind: CssPropertyDiagnosticKind::UnknownProperty,
            range: Some(range),
        })
    }
}
