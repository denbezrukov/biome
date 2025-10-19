use biome_css_syntax::CssGenericProperty;

use crate::diag::CssPropertyDiagnostic;
use crate::props::{dispatch, value::PropertyValue};

pub fn parse_property_typed(
    property: &CssGenericProperty,
) -> Result<PropertyValue, CssPropertyDiagnostic> {
    dispatch::parse_property(property)
}
