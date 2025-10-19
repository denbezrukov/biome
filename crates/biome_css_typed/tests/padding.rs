#[path = "utils.rs"]
mod utils;

use biome_css_typed::api;
use biome_css_typed::diag::CssPropertyDiagnostic;
use biome_css_typed::props::value::TypedPropertyValue;
use utils::parse_generic_property;

fn parse_padding(source: &str) -> Result<TypedPropertyValue, CssPropertyDiagnostic> {
    let property = parse_generic_property("padding", source);
    api::parse_property_typed(&property)
}

#[test]
fn padding_valid_values() {
    let results: Vec<_> = ["10px", "10px 20%", "1em 2em 3em", "1px 2px 3px 4px"]
        .into_iter()
        .map(parse_padding)
        .collect();
    insta::assert_debug_snapshot!("padding_valid_values", results);
}

#[test]
fn padding_invalid_values() {
    let results: Vec<_> = ["-1px", "1px junk", "inherit"]
        .into_iter()
        .map(parse_padding)
        .collect();
    insta::assert_debug_snapshot!("padding_invalid_values", results);
}
