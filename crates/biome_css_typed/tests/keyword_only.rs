#[path = "utils.rs"]
mod utils;

use biome_css_typed::api;
use biome_css_typed::diag::CssPropertyDiagnostic;
use biome_css_typed::props::value::TypedPropertyValue;
use utils::parse_generic_property;

fn parse_keyword_property(
    name: &str,
    source: &str,
) -> Result<TypedPropertyValue, CssPropertyDiagnostic> {
    let property = parse_generic_property(name, source);
    api::parse_property_typed(&property)
}

#[test]
fn visibility_valid_values() {
    let results: Vec<_> = ["visible", "Hidden", "collapse"]
        .into_iter()
        .map(|value| parse_keyword_property("visibility", value))
        .collect();
    insta::assert_debug_snapshot!("visibility_valid_values", results);
}

#[test]
fn visibility_invalid_values() {
    let results: Vec<_> = ["", "inherit", "visible hidden", "visible, hidden"]
        .into_iter()
        .map(|value| parse_keyword_property("visibility", value))
        .collect();
    insta::assert_debug_snapshot!("visibility_invalid_values", results);
}

#[test]
fn position_valid_values() {
    let results: Vec<_> = ["static", "Relative", "ABSOLUTE", "fixed", "sticky"]
        .into_iter()
        .map(|value| parse_keyword_property("position", value))
        .collect();
    insta::assert_debug_snapshot!("position_valid_values", results);
}

#[test]
fn position_invalid_values() {
    let results: Vec<_> = ["", "sticky fixed", "foo"]
        .into_iter()
        .map(|value| parse_keyword_property("position", value))
        .collect();
    insta::assert_debug_snapshot!("position_invalid_values", results);
}

#[test]
fn float_valid_values() {
    let results: Vec<_> = ["left", "Right", "none", "inline-start", "inline-end"]
        .into_iter()
        .map(|value| parse_keyword_property("float", value))
        .collect();
    insta::assert_debug_snapshot!("float_valid_values", results);
}

#[test]
fn float_invalid_values() {
    let results: Vec<_> = ["", "left right", "inline"]
        .into_iter()
        .map(|value| parse_keyword_property("float", value))
        .collect();
    insta::assert_debug_snapshot!("float_invalid_values", results);
}

#[test]
fn clear_valid_values() {
    let results: Vec<_> = [
        "none",
        "Left",
        "right",
        "both",
        "inline-start",
        "inline-end",
    ]
    .into_iter()
    .map(|value| parse_keyword_property("clear", value))
    .collect();
    insta::assert_debug_snapshot!("clear_valid_values", results);
}

#[test]
fn clear_invalid_values() {
    let results: Vec<_> = ["", "clear", "none left"]
        .into_iter()
        .map(|value| parse_keyword_property("clear", value))
        .collect();
    insta::assert_debug_snapshot!("clear_invalid_values", results);
}

#[test]
fn box_sizing_valid_values() {
    let results: Vec<_> = ["content-box", "BORDER-BOX"]
        .into_iter()
        .map(|value| parse_keyword_property("box-sizing", value))
        .collect();
    insta::assert_debug_snapshot!("box_sizing_valid_values", results);
}

#[test]
fn box_sizing_invalid_values() {
    let results: Vec<_> = ["", "content", "border-box border-box"]
        .into_iter()
        .map(|value| parse_keyword_property("box-sizing", value))
        .collect();
    insta::assert_debug_snapshot!("box_sizing_invalid_values", results);
}
