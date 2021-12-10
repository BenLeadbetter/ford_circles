use crate::*;
use crate::decimal::Decimal;

use gcd::Gcd;

pub fn from_centre_x(centre_x: Rational) -> Circle {
    let radius = Rational::new(1, 2*centre_x.denom().pow(2));
    Circle {
        centre: RationalPoint { x: centre_x, y: radius },
        radius,
    }
}

pub fn is_valid(circle: &Circle) -> bool {
    circle.centre.y == circle.radius
}

pub fn in_range(
    centre_x_range: RationalRange,
    radius_range: RationalRange,
) -> Circles {
    let mut ret = Circles::new();

    let q_max = 2.0 * radius_range.start.to_f64();
    let q_max = 1.0 / q_max.sqrt();
    let q_max = q_max.floor() as u64;

    let q_min = 2.0 * radius_range.end.to_f64();
    let q_min = 1.0 / q_min.sqrt();
    let q_min = (q_min + 1.0).floor() as u64;

    for q in q_min..(q_max + 1) {

        let p_limit = |bound: Rational| {
            *bound.numer() * q / *bound.denom()
        };

        let p_min = p_limit(centre_x_range.start);
        let p_max = p_limit(centre_x_range.end);

        for p in p_min..p_max {
            if p.gcd(q) == 1 {
                ret.push(from_centre_x(Rational::new(p, q)));
            }
        }
    }

    ret
}