use biome_css_syntax::{AnyCssDeclarationName, CssGenericProperty};

use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};
use crate::props::{dispatch, value::TypedPropertyValue};

pub fn parse_property_typed(
    property: &CssGenericProperty,
) -> Result<TypedPropertyValue, CssPropertyDiagnostic> {
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

    dispatch::parse_property_typed(name, &value).map_err(|mut diagnostic| {
        if diagnostic.range.is_none() {
            diagnostic.range = Some(range);
        }
        diagnostic
    })
}
