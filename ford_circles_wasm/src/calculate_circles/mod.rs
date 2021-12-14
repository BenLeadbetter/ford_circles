use crate::core::*;
use gcd::Gcd as _;

#[cfg(test)]
mod tests;

pub fn from_centre_x(centre_x: Rational) -> Circle {
    let radius = Rational::new_raw(1, 2*centre_x.denom().pow(2));
    Circle {
        centre: RationalPoint { x: centre_x, y: radius },
        radius,
    }
}

pub fn is_valid(circle: &Circle) -> bool {
    circle.centre.y == circle.radius
}

pub fn in_view(
    view: &Circle,
    resolution: Rational,
) -> Circles {
    let q_range = calculate_q_range(view, resolution);
    let mut ret = Circles::new();

    for q in q_range.0..(q_range.1 + 1) {
        let q = q as i64;
        let p_centre = *view.centre.x.numer() * q / *view.centre.x.denom();

        for (mut p, d) in [(p_centre, 1), (p_centre - 1, -1)] {
            loop {
                if (p as u64).gcd(q as u64) == 1 {    
                    let circle = from_centre_x(Rational::new_raw(p, q));
                    if !circle.intersects(&view) {
                        break;
                    }
                    ret.push(circle);
                }
                p = p + d;
            }
        }
    }

    ret
}

fn calculate_radius_range(
    view: &Circle,
    resolution: Rational,
) -> RationalRange {
    RationalRange {
        start: resolution,
        end: Rational::new(4, 1) * view.radius.pow(2) / resolution,
    }
}

fn calculate_q_range(
    view: &Circle,
    resolution: Rational,
) -> (u64, u64) {
    let radius_range = calculate_radius_range(view, resolution);

    let q_max = 2.0 * radius_range.start.to_f64();
    let q_max = 1.0 / q_max.sqrt();
    let q_max = q_max.floor() as u64;

    let q_min = 2.0 * radius_range.end.to_f64();
    let q_min = 1.0 / q_min.sqrt();
    let q_min = (q_min + 1.0).floor() as u64;

    (q_min, q_max)
}