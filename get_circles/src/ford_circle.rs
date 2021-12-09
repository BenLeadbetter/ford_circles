use crate::*;

pub fn from_centre_x(centre_x: Rational) -> Circle {
    let radius = Rational::new(1, 2*centre_x.denom().pow(2));
    Circle {
        centre: RationalPoint { x: centre_x, y: radius },
        radius,
    }
}

pub fn is_valid(circle: &Circle) -> bool {
    return circle.centre.y == circle.radius;
}

pub fn in_range(
    _centre_x_range: RationalRange,
    _radius_range: RationalRange,
) -> Circles {
    todo!();
    Circles::new()
}