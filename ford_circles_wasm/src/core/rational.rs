use crate::core::Decimal;

pub type Rational = num_rational::Ratio<i64>;

#[derive(PartialEq, Eq, Debug)]
pub struct RationalPoint {
    pub x: Rational,
    pub y: Rational,
}

pub struct RationalRange {
    pub start: Rational,
    pub end: Rational,
}

impl Decimal for Rational {
    fn to_f64(&self) -> f64 {
        (*self.numer() as f64) / (*self.denom() as f64)
    }
}