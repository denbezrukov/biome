use biome_css_syntax::CssGenericComponentValueList;
use biome_rowan::AstNode;
use biome_string_case::StrLikeExtension;

use crate::diag::{CssPropertyDiagnostic, CssPropertyDiagnosticKind};
use crate::parse::CssGenericComponentValueCursor;

use super::value::PropertyValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

impl Visibility {
    pub fn parse(
        value: &CssGenericComponentValueList,
    ) -> Result<PropertyValue, CssPropertyDiagnostic> {
        let mut cur = CssGenericComponentValueCursor::new(value);

        let first = cur.bump_non_delim().ok_or_else(|| CssPropertyDiagnostic {
            kind: CssPropertyDiagnosticKind::InvalidValue,
            range: None,
        })?;

        let token = first
            .as_any_css_value()
            .and_then(|value| value.as_css_identifier())
            .and_then(|identifier| identifier.value_token().ok())
            .ok_or_else(|| CssPropertyDiagnostic {
                kind: CssPropertyDiagnosticKind::InvalidValue,
                range: Some(first.range()),
            })?;
        let ident = token.text_trimmed().to_ascii_lowercase_cow();

        let vis = match ident.as_ref() {
            "visible" => Visibility::Visible,
            "hidden" => Visibility::Hidden,
            "collapse" => Visibility::Collapse,
            _ => {
                return Err(CssPropertyDiagnostic {
                    kind: CssPropertyDiagnosticKind::InvalidValue,
                    range: Some(first.range()),
                });
            }
        };

        if let Some(extra) = cur.peek_non_delim() {
            return Err(CssPropertyDiagnostic {
                kind: CssPropertyDiagnosticKind::UnexpectedToken,
                range: Some(extra.range()),
            });
        }

        Ok(PropertyValue::Visibility(vis))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_css_parser::{CssParserOptions, parse_css};
    use biome_rowan::AstNodeList;

    fn parse_visibility_value_list(source: &str) -> CssGenericComponentValueList {
        let css = format!("a {{ visibility: {source}; }}");
        let parse = parse_css(&css, CssParserOptions::default());
        assert!(
            !parse.has_errors(),
            "parser errors while parsing `{}`: {:?}",
            source,
            parse.diagnostics()
        );

        let root = parse.tree();
        let rule = root
            .rules()
            .iter()
            .next()
            .expect("expected at least one rule");
        let qualified_rule = rule
            .as_css_qualified_rule()
            .expect("expected a qualified rule");
        let block = qualified_rule.block().unwrap();
        let block = block
            .as_css_declaration_or_rule_block()
            .expect("expected declaration block");
        let item = block
            .items()
            .iter()
            .next()
            .expect("expected declaration item");
        let decl = item
            .as_css_declaration_with_semicolon()
            .expect("expected declaration with semicolon")
            .declaration()
            .unwrap();
        let property = decl.property().unwrap();
        let generic = property
            .as_css_generic_property()
            .expect("expected generic property");
        generic.value()
    }

    fn parse_visibility(source: &str) -> Result<PropertyValue, CssPropertyDiagnostic> {
        let list = parse_visibility_value_list(source);
        Visibility::parse(&list)
    }

    #[test]
    fn visibility_valid_values() {
        let results: Vec<_> = ["visible", "Hidden", "collapse"]
            .into_iter()
            .map(parse_visibility)
            .collect();
        insta::assert_debug_snapshot!("visibility_valid_values", results);
    }

    #[test]
    fn visibility_invalid_values() {
        let results: Vec<_> = ["", "inherit", "visible hidden", "visible, hidden"]
            .into_iter()
            .map(parse_visibility)
            .collect();
        insta::assert_debug_snapshot!("visibility_invalid_values", results);
    }
}
