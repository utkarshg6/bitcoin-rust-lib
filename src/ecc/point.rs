// The elliptic curve (y^2 = x^3 + ax + b) used in Bitcoin is called secp256k1 and it uses the particular equation:
// y^2 = x^3 + 7

use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x_opt: Option<isize>,
    // None for both x and y represents point on Infinity
    y_opt: Option<isize>,
    a: isize,
    b: isize,
}

impl Point {
    fn new(x_opt: Option<isize>, y_opt: Option<isize>, a: isize, b: isize) -> Self {
        match (x_opt, y_opt) {
            (Some(x), Some(y)) => {
                let lhs = isize::pow(y, 2);
                let rhs = isize::pow(x, 3) + (a * x) + b;
                if lhs != rhs {
                    panic!("({x}, {y}) is not on the curve.")
                }
            }
            (None, None) => {
                // Identity Element (Point is on Infinity), no validation is required
            }
            (_, _) => panic!("Both x and y coordinate should be either Some or None")
        }

        Self {
            x_opt,
            y_opt,
            a,
            b,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (self.x_opt, self.y_opt) {
            (Some(x), Some(y)) => write!(f, "Point({}, {})_{}_{}", x, y, self.a, self.b),
            (None, None) => write!(f, "Point(Infinity)"),
            (_, _) => panic!("Both x and y coordinate should be either Some or None")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ecc::point::Point;

    #[test]
    fn initialize_point_works() {
        let point = Point::new(Some(1), Some(2), 0, 3);

        assert_eq!(point, Point {
            x_opt: Some(1),
            y_opt: Some(2),
            a: 0,
            b: 3,
        })
    }

    #[test]
    #[should_panic(expected = "Both x and y coordinate should be either Some or None")]
    fn panics_when_point_with_ony_x_coordinate_is_initialized() {
        let _point = Point::new(None, Some(2), 0, 3);
    }

    #[test]
    #[should_panic(expected = "Both x and y coordinate should be either Some or None")]
    fn panics_when_point_with_ony_y_coordinate_is_initialized() {
        let _point = Point::new(Some(2), None, 0, 3);
    }

    #[test]
    #[should_panic(expected = "(-1, 1) is not on the curve.")]
    fn panics_when_point_is_not_on_the_curve() {
        let _point = Point::new(Some(-1), Some(1), 0, 3);
    }

    #[test]
    fn point_implements_display() {
        let point = Point::new(Some(1), Some(2), 0, 3);

        let subject = format!("{}", point);

        assert_eq!(&subject, "Point(1, 2)_0_3")
    }

    #[test]
    fn point_on_infinity_exists() {
        let point = Point::new(None, None, 0, 3);
        let point_display = format!("{}", point);

        assert_eq!(point, Point {
            x_opt: None,
            y_opt: None,
            a: 0,
            b: 3,
        });
        assert_eq!(&point_display, "Point(Infinity)")
    }
}


