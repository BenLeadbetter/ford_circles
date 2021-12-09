use crate::decimal::Decimal;

pub type Rational = num_rational::Ratio<u64>;

#[derive(PartialEq, Debug)]
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