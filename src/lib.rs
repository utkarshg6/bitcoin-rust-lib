use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
struct FiniteField {
    num: usize,
    prime: usize,
}

impl FiniteField {
    fn new(num: usize, prime: usize) -> Self {
        if num >= prime {
            panic!("Num {num} not in field range 0 to {prime}");
        }

        Self {
            num,
            prime,
        }
    }
}

impl Display for FiniteField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

#[cfg(test)]
mod tests {
    use crate::FiniteField;

    #[test]
    fn initialize_field_element_works() {
        let subject = FiniteField::new(3, 5);

        assert_eq!(subject, FiniteField {
            num: 3,
            prime: 5,
        })
    }

    #[test]
    #[should_panic(expected = "Num 5 not in field range 0 to 5")]
    fn initializing_field_panics_if_num_is_greater_than_or_equal_to_prime() {
        let _subject = FiniteField::new(5, 5);
    }

    #[test]
    fn field_element_implements_display() {
        let field_element = FiniteField::new(3, 5);

        let subject = format!("{}", field_element);

        assert_eq!(&subject, "FieldElement_5(3)")
    }
}
