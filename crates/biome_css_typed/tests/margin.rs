#[path = "utils.rs"]
mod utils;

use biome_css_typed::api;
use biome_css_typed::diag::CssPropertyDiagnostic;
use biome_css_typed::props::value::TypedPropertyValue;
use utils::parse_generic_property;

fn parse_margin(source: &str) -> Result<TypedPropertyValue, CssPropertyDiagnostic> {
    let property = parse_generic_property("margin", source);
    api::parse_property_typed(&property)
}

#[test]
fn margin_valid_values() {
    let results: Vec<_> = [
        "auto",
        "10px",
        "10px auto",
        "1em 2em 3em",
        "-4px 5%",
        "1px 2px 3px 4px",
    ]
    .into_iter()
    .map(parse_margin)
    .collect();

    insta::assert_debug_snapshot!("margin_valid_values", results);
}

#[test]
fn margin_invalid_values() {
    let results: Vec<_> = ["junk", "1px 2px 3px 4px junk", "inherit"]
        .into_iter()
        .map(parse_margin)
        .collect();

    insta::assert_debug_snapshot!("margin_invalid_values", results);
}
