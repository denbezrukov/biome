use biome_css_syntax::CssGenericComponentValueList;

use crate::diag::CssPropertyDiagnostic;
use crate::props::{dispatch, value::PropertyValue};

pub fn parse_property_typed(
    name: &str,
    value: &CssGenericComponentValueList,
) -> Result<PropertyValue, CssPropertyDiagnostic> {
    dispatch::parse_property(name, value)
}
