use super::{
    box_sizing::BoxSizing, clear::Clear, float::Float, margin::MarginShorthand,
    padding::PaddingShorthand, position::Position, visibility::Visibility,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TypedPropertyValue {
    Margin(MarginShorthand),
    Padding(PaddingShorthand),
    Visibility(Visibility),
    Position(Position),
    Float(Float),
    Clear(Clear),
    BoxSizing(BoxSizing),
}
