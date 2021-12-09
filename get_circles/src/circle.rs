use crate::rational::Rational;
use crate::rational::RationalPoint;

pub struct Circle {
    pub radius: Rational,
    pub centre: RationalPoint,
}
pub type Circles = std::vec::Vec<Circle>;