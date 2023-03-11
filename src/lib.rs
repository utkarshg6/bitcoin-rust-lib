#[derive(Debug, PartialEq)]
struct FiniteField {
    num: usize,
    prime: usize,
}

impl FiniteField {
    fn new(num: usize, prime: usize) -> Self {
        Self {
            num,
            prime
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::FiniteField;

    #[test]
    fn initialize_field_element_works() {
        let finite_field_element = FiniteField::new(3, 5);

        assert_eq!(finite_field_element, FiniteField {
            num: 3,
            prime: 5
        })
    }
}
