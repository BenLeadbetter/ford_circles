use crate::rational::Rational;
use crate::rational::RationalPoint;

#[derive(PartialEq, Eq, Debug)]
pub struct Circle {
    pub radius: Rational,
    pub centre: RationalPoint,
}
pub type Circles = std::vec::Vec<Circle>;

impl Circle {
    pub fn intersects(&self, other: &Circle) -> bool {
        self.sq_distance_between_centres(other) < (self.radius + other.radius).pow(2)
    }

    pub fn sq_distance_between_centres(&self, other: &Circle) -> Rational {
        (self.centre.x - other.centre.x).pow(2) + (self.centre.y - other.centre.y).pow(2)
    }
}

impl std::cmp::PartialOrd for Circle {
    fn partial_cmp(&self, other: &Circle) -> Option<std::cmp::Ordering> {
        if self.sq_distance_between_centres(other) == Rational::new_raw(0, 1) {
            if self.radius < other.radius {
                return Some(std::cmp::Ordering::Less);
            }  else if self.radius > other.radius {
                return Some(std::cmp::Ordering::Greater);
            } else {
                return Some(std::cmp::Ordering::Equal);
            }
        } else if self.centre.x < other.centre.x {
            return Some(std::cmp::Ordering::Less);
        } else if self.centre.x < other.centre.x {
            return Some(std::cmp::Ordering::Greater);
        } else {
            return Some(std::cmp::Ordering::Equal);
        }
    }
}

impl std::cmp::Ord for Circle {
    fn cmp(&self, other: &Circle) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}