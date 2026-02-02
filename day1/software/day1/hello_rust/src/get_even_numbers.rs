pub fn get_even_numbers(numbers: &[i32]) -> String {
    let mut sorted = numbers.to_vec();
    sorted.sort();

    let mut sorted_numbers = Vec::new();
    for &num in &sorted {
        if num % 2 == 0 {
            sorted_numbers.push(num);
        }
    }

    sorted_numbers.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(" - ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_even_numbers_positive() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(get_even_numbers(&numbers), "2 - 4 - 6");
    }
    #[test]
    fn test_get_even_numbers_negative() {
        let numbers = vec![-8, -7, -6, -5, -4, -3, -2];
        assert_eq!(get_even_numbers(&numbers), "-8 - -6 - -4 - -2");
    }

    #[test]
    fn test_get_even_numbers_mixed() {
        let numbers = vec![-2, -3, -8, 7, 5, 3];
        assert_eq!(get_even_numbers(&numbers), "-8 - -2");
    }
}
