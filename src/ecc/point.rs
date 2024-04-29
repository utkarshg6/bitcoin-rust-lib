// The elliptic curve (y^2 = x^3 + ax + b) used in Bitcoin is called secp256k1 and it uses the particular equation:
// y^2 = x^3 + 7

use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
    a: isize,
    b: isize,
}

impl Point {
    fn new(x: isize, y: isize, a: isize, b: isize) -> Self {
        let lhs = isize::pow(y, 2);
        let rhs = isize::pow(x, 3) + (a * x) + b;
        if lhs != rhs {
            panic!("({x}, {y}) is not on the curve.")
        }

        Self {
            x,
            y,
            a,
            b,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})_{}_{}", self.x, self.y, self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::ecc::point::Point;

    #[test]
    fn initialize_point_works() {
        let point = Point::new(1, 2, 0, 3);

        assert_eq!(point, Point {
            x: 1,
            y: 2,
            a: 0,
            b: 3,
        })
    }

    #[test]
    #[should_panic(expected = "(-1, 1) is not on the curve.")]
    fn panics_when_point_is_not_on_the_curve() {
        let _point = Point::new(-1, 1, 0, 3);
    }

    #[test]
    fn point_implements_display() {
        let point = Point::new(1, 2, 0, 3);

        let subject = format!("{}", point);

        assert_eq!(&subject, "Point(1, 2)_0_3")
    }
}


