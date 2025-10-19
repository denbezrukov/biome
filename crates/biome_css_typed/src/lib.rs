//! Typed CSS property parser and analyzer for Biome.
//!
//! This crate builds on top of `biome_css_syntax` and provides
//! a strongly-typed representation of CSS property values for use
//! in analyzers and lint rules.

pub mod api;
pub mod diag;
pub mod parse;
pub mod props;
pub mod util;
