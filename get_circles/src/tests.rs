use super::*;

#[test]
fn from_centre_x_creates_valid_ford_circle() {
    let circle = ford_circle::from_centre_x(Rational::new(2,3));
    assert_eq!(ford_circle::is_valid(&circle), true);
}

#[test]
fn view_radius_one_centre_zero_resolution_half() {
    let mut circles = ford_circle::in_view(
        &Circle {
            centre: RationalPoint { 
                x: Rational::new(0, 1),
                y: Rational::new(0, 1),
            },
            radius: Rational::new(1, 1),
        },
        Rational::new(1,2)
    );
    circles.sort();

    let mut expected_circles = std::vec![
        ford_circle::from_centre_x(Rational::new(-1, 1)),
        ford_circle::from_centre_x(Rational::new(0, 1)),
        ford_circle::from_centre_x(Rational::new(1, 1)),
    ];
    expected_circles.sort();

    assert_eq!(circles, expected_circles);
}

#[test]
fn view_radius_two_centre_zero_resolution_half() {
    let mut circles = ford_circle::in_view(
        &Circle {
            centre: RationalPoint { 
                x: Rational::new(0, 1),
                y: Rational::new(0, 1),
            },
            radius: Rational::new(2, 1),
        },
        Rational::new(1,2)
    );
    circles.sort();

    let mut expected_circles = std::vec![
        ford_circle::from_centre_x(Rational::new(-2, 1)),
        ford_circle::from_centre_x(Rational::new(-1, 1)),
        ford_circle::from_centre_x(Rational::new(0, 1)),
        ford_circle::from_centre_x(Rational::new(1, 1)),
        ford_circle::from_centre_x(Rational::new(2, 1)),
    ];
    expected_circles.sort();

    assert_eq!(circles, expected_circles);
}

#[test]
fn view_radius_one_centre_zero_resolution_an_eighth() {
    let mut circles = ford_circle::in_view(
        &Circle {
            centre: RationalPoint { 
                x: Rational::new(0, 1),
                y: Rational::new(0, 1),
            },
            radius: Rational::new(1, 1),
        },
        Rational::new(1,8)
    );
    circles.sort();

    let mut expected_circles = std::vec![
        ford_circle::from_centre_x(Rational::new(-1, 1)),
        ford_circle::from_centre_x(Rational::new(0, 1)),
        ford_circle::from_centre_x(Rational::new(1, 1)),
        ford_circle::from_centre_x(Rational::new(1, 2)),
        ford_circle::from_centre_x(Rational::new(-1, 2)),
    ];
    expected_circles.sort();

    assert_eq!(circles, expected_circles);
}

#[test]
fn view_radius_one_twenty_fourth_centre_one_fifth_resolution_a_hundredth() {
    let mut circles = ford_circle::in_view(
        &Circle {
            centre: RationalPoint { 
                x: Rational::new(1, 5),
                y: Rational::new(0, 1),
            },
            radius: Rational::new(1, 24),
        },
        Rational::new(1, 100)
    );
    circles.sort();

    let mut expected_circles = std::vec![
        ford_circle::from_centre_x(Rational::new(0, 1)),
        ford_circle::from_centre_x(Rational::new(1, 6)),
        ford_circle::from_centre_x(Rational::new(1, 5)),
        ford_circle::from_centre_x(Rational::new(1, 4)),
    ];
    expected_circles.sort();

    assert_eq!(circles, expected_circles);
}

#[test]
fn large_circles_not_included_for_correct_resolution() {
    let mut circles = ford_circle::in_view(
        &Circle {
            centre: RationalPoint { 
                x: Rational::new(1, 5),
                y: Rational::new(0, 1),
            },
            radius: Rational::new(1, 24),
        },
        Rational::new(1, 50)
    );
    circles.sort();

    let mut expected_circles = std::vec![
        ford_circle::from_centre_x(Rational::new(1, 5)),
        ford_circle::from_centre_x(Rational::new(1, 4)),
    ];
    expected_circles.sort();

    assert_eq!(circles, expected_circles);
}