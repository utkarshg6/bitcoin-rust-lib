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
}
