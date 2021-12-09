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

#[test]
fn first_order_circles_between_zero_and_three_correct_circles() {
    let circles = ford_circle::in_range(
        RationalRange {
            start: Rational::new(0, 1),
            end: Rational::new(3, 1),
        },
        RationalRange {
            start: Rational::new(1, 2),
            end: Rational::new(1, 1),
        },
    );

    let expected_circles = std::vec![
        ford_circle::from_centre_x(Rational::new(0, 1)),
        ford_circle::from_centre_x(Rational::new(1, 1)),
        ford_circle::from_centre_x(Rational::new(2, 1)),
    ];

    for circle in circles {
        if !expected_circles.contains(&circle) {
            panic!();
        }
    }
}

#[test]
fn circles_between_zero_and_one_radius_greater_than_quarter() {
    let circles = ford_circle::in_range(
        RationalRange{
            start: Rational::new(0, 1),
            end: Rational::new(1, 1),
        },
        RationalRange{
            start: Rational::new(1, 4),
            end: Rational::new(1, 1),
        },
    );

    let expected_circles = std::vec![
        ford_circle::from_centre_x(Rational::new(0, 1)),
        ford_circle::from_centre_x(Rational::new(1, 1)),
        ford_circle::from_centre_x(Rational::new(1, 2)),
        ford_circle::from_centre_x(Rational::new(1, 3)),
        ford_circle::from_centre_x(Rational::new(2, 3)),
        ford_circle::from_centre_x(Rational::new(1, 4)),
        ford_circle::from_centre_x(Rational::new(3, 4)),
    ];

    for circle in circles {
        if !expected_circles.contains(&circle) {
            panic!();
        }
    }
}

#[test]
fn circles_between_one_and_two_radius_between_a_tenth_and_a_quarter() {
    let circles = ford_circle::in_range(
        RationalRange{
            start: Rational::new(1, 1),
            end: Rational::new(2, 1),
        },
        RationalRange{
            start: Rational::new(1, 10),
            end: Rational::new(1, 4),
        },
    );

    let expected_circles = std::vec![
        ford_circle::from_centre_x(Rational::new(5, 4)),
        ford_circle::from_centre_x(Rational::new(7, 4)),
        ford_circle::from_centre_x(Rational::new(6, 5)),
        ford_circle::from_centre_x(Rational::new(7, 5)),
        ford_circle::from_centre_x(Rational::new(8, 5)),
        ford_circle::from_centre_x(Rational::new(9, 5)),
        ford_circle::from_centre_x(Rational::new(7, 6)),
        ford_circle::from_centre_x(Rational::new(11, 6)),
        ford_circle::from_centre_x(Rational::new(8, 7)),
        ford_circle::from_centre_x(Rational::new(9, 7)),
        ford_circle::from_centre_x(Rational::new(10, 7)),
        ford_circle::from_centre_x(Rational::new(11, 7)),
        ford_circle::from_centre_x(Rational::new(12, 7)),
        ford_circle::from_centre_x(Rational::new(13, 7)),
        ford_circle::from_centre_x(Rational::new(9, 8)),
        ford_circle::from_centre_x(Rational::new(11, 8)),
        ford_circle::from_centre_x(Rational::new(13, 8)),
        ford_circle::from_centre_x(Rational::new(15, 8)),
        ford_circle::from_centre_x(Rational::new(10, 9)),
        ford_circle::from_centre_x(Rational::new(11, 9)),
        ford_circle::from_centre_x(Rational::new(13, 9)),
        ford_circle::from_centre_x(Rational::new(14, 9)),
        ford_circle::from_centre_x(Rational::new(16, 9)),
        ford_circle::from_centre_x(Rational::new(17, 9)),
    ];

    println!("{:?}", circles);

    for circle in circles {
        if !expected_circles.contains(&circle) {
            panic!();
        }
    }
}