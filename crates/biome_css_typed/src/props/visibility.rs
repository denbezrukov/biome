use biome_css_syntax::CssGenericComponentValueList;

use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};

use super::value::PropertyValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

impl Visibility {
    pub(crate) fn parse(
        value: &CssGenericComponentValueList,
    ) -> Result<PropertyValue, CssPropertyDiagnostic> {
        let _ = value;
        Err(CssPropertyDiagnostic {
            kind: CssPropertyDiagnosticKind::InvalidValue,
            range: None,
        })
    }
}
