pub type Rational = num_rational::Ratio<u64>;

pub struct RationalPoint {
    pub x: Rational,
    pub y: Rational,
}

pub struct RationalRange {
    pub start: Rational,
    pub end: Rational,
}