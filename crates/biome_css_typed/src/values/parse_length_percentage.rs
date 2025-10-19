use super::parse_length::parse_length;
use super::parse_percentage::parse_percentage;
use super::percent::LengthPercentage;
use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};
use crate::parse::CssGenericComponentValueCursor;
use crate::parse::ext::CssNodeExt;

pub fn parse_length_percentage(
    cur: &mut CssGenericComponentValueCursor,
) -> Result<LengthPercentage, CssPropertyDiagnostic> {
    let checkpoint = cur.checkpoint();
    match parse_percentage(cur) {
        Ok(percentage) => Ok(LengthPercentage::Percentage(percentage)),
        Err(first_err) => {
            let first_range = first_err.range;
            cur.restore(checkpoint);
            match parse_length(cur) {
                Ok(length) => Ok(LengthPercentage::Length(length)),
                Err(second_err) => Err(CssPropertyDiagnostic {
                    kind: CssPropertyDiagnosticKind::InvalidValue,
                    range: first_range
                        .or(second_err.range)
                        .or_else(|| cur.peek_non_delim().and_then(|node| node.range_opt())),
                }),
            }
        }
    }
}
