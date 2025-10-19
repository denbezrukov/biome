use super::dimension::Length;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LengthPercentage {
    Length(Length),
    Percentage(Percentage),
}

impl LengthPercentage {
    pub fn is_negative(&self) -> bool {
        match self {
            LengthPercentage::Length(length) => length.value < 0.0,
            LengthPercentage::Percentage(percentage) => percentage.0 < 0.0,
        }
    }
}
