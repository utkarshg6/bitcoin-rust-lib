#[derive(Debug)]
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

impl PartialEq<Self> for FiniteField {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

#[cfg(test)]
mod tests {
    use crate::FiniteField;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn initialize_field_element_works() {
        let finite_field_element = FiniteField::new(3, 5);

        assert_eq!(finite_field_element, FiniteField {
            num: 3,
            prime: 5
        })
    }
}
