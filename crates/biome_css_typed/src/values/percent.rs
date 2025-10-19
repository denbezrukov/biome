use super::dimension::Length;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LengthPercentage {
    Length(Length),
    Percentage(Percentage),
}
