use super::visibility::Visibility;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyValue {
    Visibility(Visibility),
    // Display(super::display::Display),  // later
}
