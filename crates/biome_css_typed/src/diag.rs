use biome_diagnostics::{Diagnostic, Location, Severity, console::fmt::Formatter};
use biome_text_size::TextRange;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CssPropertyDiagnostic {
    pub kind: CssPropertyDiagnosticKind,
    pub range: Option<TextRange>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssPropertyDiagnosticKind {
    UnknownProperty,
    InvalidValue,
    UnexpectedToken,
    DuplicateComponent,
}

impl CssPropertyDiagnosticKind {
    fn message(self) -> &'static str {
        match self {
            CssPropertyDiagnosticKind::UnknownProperty => "Unknown CSS property.",
            CssPropertyDiagnosticKind::InvalidValue => "Invalid value for CSS property.",
            CssPropertyDiagnosticKind::UnexpectedToken => "Unexpected token in CSS property value.",
            CssPropertyDiagnosticKind::DuplicateComponent => {
                "Duplicate component in CSS property value."
            }
        }
    }
}

impl Diagnostic for CssPropertyDiagnostic {
    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self, fmt: &mut Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(self.kind.message())
    }

    fn description(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.kind.message())
    }

    fn location(&self) -> Location<'_> {
        let mut builder = Location::builder();
        if let Some(range) = &self.range {
            builder = builder.span(range);
        }
        builder.build()
    }
}
