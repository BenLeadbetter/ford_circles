use super::*;

#[test]
fn from_centre_x_creates_valid_ford_circle() {
    let circle = ford_circle::from_centre_x(Rational::new(2,3));
    assert_eq!(ford_circle::is_valid(&circle), true);
}

#[test]
fn first_order_circles_between_zero_and_three_returns_three_circles() {
    let circles = ford_circle::in_range(
        RationalRange{
            start: Rational::new(0, 1),
            end: Rational::new(3, 1),
        },
        RationalRange{
            start: Rational::new(1, 2),
            end: Rational::new(1, 1),
        },
    );
    assert_eq!(circles.len(), 3);
}
