use super::{padding::PaddingShorthand, visibility::Visibility};

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Visibility(Visibility),
    PaddingShorthand(PaddingShorthand),
    // Display(super::display::Display),  // later
}
