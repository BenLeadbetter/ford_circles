use crate::rational::Rational;
use crate::rational::RationalPoint;

#[derive(PartialEq, Debug)]
pub struct Circle {
    pub radius: Rational,
    pub centre: RationalPoint,
}
pub type Circles = std::vec::Vec<Circle>;