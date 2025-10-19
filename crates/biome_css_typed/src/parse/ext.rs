use biome_css_syntax::{AnyCssDimension, AnyCssGenericComponentValue, AnyCssValue};
use biome_rowan::AstNode;
use biome_text_size::TextRange;

#[allow(dead_code)]
pub(crate) trait CssNodeExt {
    /// True if this node is a generic delimiter.
    fn is_delim(&self) -> bool;

    /// Case-insensitive check if the identifier equals the given ASCII string.
    fn ident_eq_ascii(&self, needle: &str) -> bool;

    /// Case-insensitive check if the dimension unit equals the given ASCII string.
    fn unit_eq_ascii(&self, needle: &str) -> bool;

    /// Returns (value, is_integer) if this node is a number.
    fn number(&self) -> Option<(f64, bool)>;

    /// Returns a best-effort source range for diagnostics.
    fn range_opt(&self) -> Option<TextRange>;
}

impl CssNodeExt for AnyCssGenericComponentValue {
    fn is_delim(&self) -> bool {
        self.as_css_generic_delimiter().is_some()
    }

    fn ident_eq_ascii(&self, needle: &str) -> bool {
        match self.as_any_css_value() {
            Some(AnyCssValue::CssIdentifier(identifier)) => {
                identifier.value_token().ok().map_or(false, |token| {
                    token.text_trimmed().eq_ignore_ascii_case(needle)
                })
            }
            Some(AnyCssValue::CssCustomIdentifier(identifier)) => {
                identifier.value_token().ok().map_or(false, |token| {
                    token.text_trimmed().eq_ignore_ascii_case(needle)
                })
            }
            Some(AnyCssValue::CssDashedIdentifier(identifier)) => {
                identifier.value_token().ok().map_or(false, |token| {
                    token.text_trimmed().eq_ignore_ascii_case(needle)
                })
            }
            _ => false,
        }
    }

    fn unit_eq_ascii(&self, needle: &str) -> bool {
        match self.as_any_css_value() {
            Some(AnyCssValue::AnyCssDimension(dimension)) => match dimension {
                AnyCssDimension::CssPercentage(percentage) => {
                    percentage.percent_token().ok().map_or(false, |token| {
                        token.text_trimmed().eq_ignore_ascii_case(needle)
                    })
                }
                AnyCssDimension::CssRegularDimension(dimension) => {
                    dimension.unit_token().ok().map_or(false, |token| {
                        token.text_trimmed().eq_ignore_ascii_case(needle)
                    })
                }
                AnyCssDimension::CssUnknownDimension(dimension) => {
                    dimension.unit_token().ok().map_or(false, |token| {
                        token.text_trimmed().eq_ignore_ascii_case(needle)
                    })
                }
            },
            _ => false,
        }
    }

    fn number(&self) -> Option<(f64, bool)> {
        let number = self
            .as_any_css_value()
            .and_then(AnyCssValue::as_css_number)?;
        let token = number.value_token().ok()?;
        parse_number_literal(token.text_trimmed())
    }

    fn range_opt(&self) -> Option<TextRange> {
        Some(self.range())
    }
}

#[allow(dead_code)]
fn parse_number_literal(text: &str) -> Option<(f64, bool)> {
    let value = text.parse::<f64>().ok()?;
    Some((value, is_integer_literal(text)))
}

#[allow(dead_code)]
fn is_integer_literal(text: &str) -> bool {
    let without_sign = if let Some(rest) = text.strip_prefix('+') {
        rest
    } else if let Some(rest) = text.strip_prefix('-') {
        rest
    } else {
        text
    };

    !without_sign.is_empty() && without_sign.chars().all(|ch| ch.is_ascii_digit())
}
