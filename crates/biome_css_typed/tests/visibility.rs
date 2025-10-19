#[path = "utils.rs"]
mod utils;

use biome_css_typed::diag::CssPropertyDiagnostic;
use biome_css_typed::props::value::PropertyValue;
use biome_css_typed::props::visibility::Visibility;
use utils::parse_generic_property;

fn parse_visibility(source: &str) -> Result<PropertyValue, CssPropertyDiagnostic> {
    let property = parse_generic_property("visibility", source);
    let value = property.value();
    Visibility::parse(&value)
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
