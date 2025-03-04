// Tests are important to ensure that your code does what you think it should do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    println!("Is 4 even? {}", is_even(4));
    println!("Is 7 even? {}", is_even(7));
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn you_can_assert() {
        // Test the function `is_even` with some values
        assert!(is_even(2), "2 should be even");
        assert!(is_even(4), "4 should be even");
        assert!(!is_even(3), "3 should not be even");
        assert!(!is_even(5), "5 should not be even");
    }
}
