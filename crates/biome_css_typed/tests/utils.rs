use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::CssGenericProperty;
use biome_rowan::AstNodeList;

pub fn parse_generic_property(property_name: &str, source: &str) -> CssGenericProperty {
    let css = format!("a {{ {property_name}: {source}; }}");
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
    generic.clone()
}
