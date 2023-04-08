use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug, PartialEq)]
struct FiniteField {
    num: usize,
    prime: usize,
}

impl FiniteField {
    fn new(num: usize, prime: usize) -> Self {
        /*
            Why fields have to have a prime power number of elements?

            No matter what k you choose, as long as it’s greater than 0,
            multiplying the entire set by k will result in the same set
            as you started with.

            Intuitively, the fact that we have a prime order results in
            every element of a finite field being equivalent. If the
            order of the set was a composite number, multiplying the set
            by one of the divisors would result in a smaller set.
         */

        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }

        Self {
            num,
            prime,
        }
    }

    fn pow(self, mut exp: i32) -> Self {
        /*
            Fun Fact: If you raise any element of the field
            with p-1, it'll be equal to 1.

            1^(p – 1) = 2^(p – 1) = 3^(p – 1) = 4^(p – 1) = ... = (p – 1)^(p – 1) = 1

            It is represented by a^(p-1) = 1
         */

        while exp < 0 {
            // a^(-exp) = a^(-exp) * 1 = a^(-exp) * a^(p-1)
            exp += self.prime as i32 - 1
        };

        let num = usize::pow(self.num, exp as u32);

        FiniteField::new(
            num % self.prime,
            self.prime,
        )
    }
}

impl Display for FiniteField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}


impl Add for FiniteField {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot add two numbers in different fields {} and {}.", self.prime, rhs.prime);
        }

        FiniteField::new(
            (self.num + rhs.num) % self.prime,
            self.prime,
        )
    }
}

impl Sub for FiniteField {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot subtract two numbers in different fields {} and {}.", self.prime, rhs.prime);
        }

        let a_minus_b = match self.num > rhs.num {
            true => self.num - rhs.num,
            false => self.prime + self.num - rhs.num // -n = p - n = p + (a - b) = p + a - b
        };

        FiniteField::new(
            a_minus_b % self.prime,
            self.prime,
        )
    }
}

impl Mul for FiniteField {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot multiply two numbers in different fields {} and {}.", self.prime, rhs.prime)
        }

        FiniteField::new(
            (self.num * rhs.num) % self.prime,
            self.prime,
        )
    }
}

impl Div for FiniteField {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot divide two numbers in different fields {} and {}.", self.prime, rhs.prime)
        }

        // a / b = a * (1/b) = a * b^(-1)
        let rhs_inverse = FiniteField::pow(rhs.clone(), -1);

        self * rhs_inverse
    }
}

#[cfg(test)]
mod tests {
    use crate::ecc::finite_field::FiniteField;

    #[test]
    fn initialize_field_element_works() {
        let subject = FiniteField::new(3, 5);

        assert_eq!(subject, FiniteField {
            num: 3,
            prime: 5,
        })
    }

    #[test]
    #[should_panic(expected = "Num 6 not in field range 0 to 4")]
    fn initializing_field_panics_if_num_is_greater_than_or_equal_to_prime() {
        let _subject = FiniteField::new(6, 5);
    }

    #[test]
    fn field_element_implements_display() {
        let field_element = FiniteField::new(3, 5);

        let subject = format!("{}", field_element);

        assert_eq!(&subject, "FieldElement_5(3)")
    }

    #[test]
    fn field_elements_can_be_added() {
        let a = FiniteField::new(1, 5);
        let b = FiniteField::new(2, 5);

        let result = a + b;

        let expected = FiniteField::new(3, 5);
        assert_eq!(result, expected);
    }

    #[test]
    fn field_elements_with_sum_greater_than_range_can_be_added() {
        let a = FiniteField::new(2, 5);
        let b = FiniteField::new(4, 5);

        let result = a + b;

        let expected = FiniteField::new(1, 5);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Cannot add two numbers in different fields 5 and 7.")]
    fn field_elements_of_different_fields_can_not_be_added() {
        let a = FiniteField::new(1, 5);
        let b = FiniteField::new(2, 7);

        let _result = a + b;
    }

    #[test]
    fn field_elements_can_be_subtracted() {
        let a = FiniteField::new(3, 5);
        let b = FiniteField::new(2, 5);

        let result = a - b;

        let expected = FiniteField::new(1, 5);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Cannot subtract two numbers in different fields 5 and 7.")]
    fn field_elements_of_different_fields_cannot_be_subtracted() {
        let a = FiniteField::new(2, 5);
        let b = FiniteField::new(1, 7);

        let _result = a - b;
    }

    #[test]
    fn field_elements_with_a_negative_result_can_be_calculated() {
        let a = FiniteField::new(2, 5);
        let b = FiniteField::new(3, 5);

        let result = a - b;

        let expected = FiniteField::new(4, 5); // -1 % 5 = 4
        assert_eq!(result, expected);
    }

    #[test]
    fn field_elements_can_be_multiplied() {
        let a = FiniteField::new(2, 7);
        let b = FiniteField::new(3, 7);

        let result = a * b;

        let expected = FiniteField::new(6, 7);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Cannot multiply two numbers in different fields 5 and 7.")]
    fn field_elements_of_different_fields_cannot_be_multiplied() {
        let a = FiniteField::new(2, 5);
        let b = FiniteField::new(3, 7);

        let _result = a * b;
    }

    #[test]
    fn field_elements_with_product_greater_than_range_can_be_calculated() {
        let a = FiniteField::new(2, 5);
        let b = FiniteField::new(3, 5);

        let result = a * b;

        let expected = FiniteField::new(1, 5);
        assert_eq!(result, expected);
    }

    #[test]
    fn exponent_of_a_field_can_be_calculated() {
        let a = FiniteField::new(2, 3);

        let result = FiniteField::pow(a.clone(), 3);

        let expected = FiniteField::new(2, 3);
        assert_eq!(result, expected);
    }

    #[test]
    fn negative_exponent_of_a_field_can_be_calculated() {
        let a = FiniteField::new(7, 13);

        let result = FiniteField::pow(a.clone(), -3);

        let expected = FiniteField::new(8, 13);
        assert_eq!(result, expected);
    }

    #[test]
    fn field_elements_can_be_divided() {
        // For a field of 5
        // 2 * 3 = 6 = 6 % 5 = 1
        // 1 / 3 = 2
        let a = FiniteField::new(1, 5);
        let b = FiniteField::new(3, 5);

        let result = a / b;

        let expected = FiniteField::new(2, 5);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Cannot divide two numbers in different fields 5 and 7.")]
    fn field_elements_of_different_fields_cannot_be_divided() {
        let a = FiniteField::new(2, 5);
        let b = FiniteField::new(3, 7);

        let _result = a / b;
    }
}
