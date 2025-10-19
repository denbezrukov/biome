use biome_css_syntax::CssGenericComponentValueList;
use biome_rowan::AstNodeList;

use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};
use crate::parse::CssGenericComponentValueCursor;
use crate::parse::ext::CssNodeExt;
use crate::values::r#box::Box4;
use crate::values::parse_length_percentage::parse_length_percentage;
use crate::values::percent::LengthPercentage;

use super::value::PropertyValue;

#[derive(Debug, Clone, PartialEq)]
pub struct PaddingShorthand(pub Box4<LengthPercentage>);

impl PaddingShorthand {
    pub fn parse(
        value: &CssGenericComponentValueList,
    ) -> Result<PropertyValue, CssPropertyDiagnostic> {
        let mut cur = CssGenericComponentValueCursor::new(value);
        let mut values = Vec::with_capacity(4);

        for _ in 0..4 {
            cur.skip_delims();

            if cur.peek_non_delim().is_none() {
                break;
            }

            let checkpoint = cur.checkpoint();
            match parse_length_percentage(&mut cur) {
                Ok(length_percentage) => {
                    if length_percentage.is_negative() {
                        let range = checkpoint
                            .peek_non_delim()
                            .and_then(|node| node.range_opt());
                        return Err(CssPropertyDiagnostic {
                            kind: CssPropertyDiagnosticKind::InvalidValue,
                            range,
                        });
                    }

                    values.push(length_percentage);
                }
                Err(err) => {
                    if values.is_empty() {
                        return Err(err);
                    }

                    cur.restore(checkpoint);
                    break;
                }
            }
        }

        if values.is_empty() {
            let range = value.iter().next().and_then(|node| node.range_opt());
            return Err(CssPropertyDiagnostic {
                kind: CssPropertyDiagnosticKind::InvalidValue,
                range,
            });
        }

        cur.skip_delims();
        if let Some(extra) = cur.peek_non_delim() {
            return Err(CssPropertyDiagnostic {
                kind: CssPropertyDiagnosticKind::UnexpectedToken,
                range: extra.range_opt(),
            });
        }

        let shorthand = Box4::new(values).expect("padding expects between 1 and 4 values");

        Ok(PropertyValue::PaddingShorthand(PaddingShorthand(shorthand)))
    }
}
