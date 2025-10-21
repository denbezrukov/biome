use biome_css_syntax::CssGenericComponentValueList;
use biome_rowan::AstNodeList;

use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};
use crate::parse::CssGenericComponentValueCursor;
use crate::parse::ext::CssNodeExt;
use crate::values::r#box::Box4;
use crate::values::parse_length_percentage::parse_length_percentage;
use crate::values::percent::LengthPercentage;

#[derive(Debug, Clone, PartialEq)]
pub enum MarginSideValue {
    Auto,
    LengthPercentage(LengthPercentage),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MarginShorthand(pub Box4<MarginSideValue>);

pub fn parse_margin_side(
    cur: &mut CssGenericComponentValueCursor,
) -> Result<MarginSideValue, CssPropertyDiagnostic> {
    cur.skip_delims();

    let Some(node) = cur.peek_non_delim() else {
        return Err(CssPropertyDiagnostic {
            kind: CssPropertyDiagnosticKind::InvalidValue,
            range: None,
        });
    };

    if node.ident_eq_ascii("auto") {
        cur.bump_non_delim();
        return Ok(MarginSideValue::Auto);
    }

    match parse_length_percentage(cur) {
        Ok(length_percentage) => Ok(MarginSideValue::LengthPercentage(length_percentage)),
        Err(_) => Err(CssPropertyDiagnostic {
            kind: CssPropertyDiagnosticKind::InvalidValue,
            range: node.range_opt(),
        }),
    }
}

impl MarginShorthand {
    pub fn parse(
        value: &CssGenericComponentValueList,
    ) -> Result<MarginShorthand, CssPropertyDiagnostic> {
        let mut cur = CssGenericComponentValueCursor::new(value);
        let mut values = Vec::with_capacity(4);

        for _ in 0..4 {
            cur.skip_delims();

            if cur.peek_non_delim().is_none() {
                break;
            }

            let checkpoint = cur.checkpoint();
            match parse_margin_side(&mut cur) {
                Ok(value) => values.push(value),
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

        let shorthand = Box4::new(values).expect("margin expects between 1 and 4 values");

        Ok(MarginShorthand(shorthand))
    }
}
