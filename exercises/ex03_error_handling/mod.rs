// Exercise 03: Error Handling with Option and Result

pub fn exercise() {
    println!("\n=== Exercise 03: Error Handling ===");

    challenge_1();
    challenge_2();
    challenge_3();
}

fn challenge_1() {
    // TODO: Complete this function using Option
    // Find the first element in a vector that's greater than 50
    // Return Some(value) if found, None otherwise

    let numbers = vec![10, 25, 60, 30, 80];
    // match find_first_greater_than_50(&numbers) {
    //     Some(val) => println!("Found: {}", val),
    //     None => println!("Not found"),
    // }
}

fn find_first_greater_than_50(numbers: &Vec<i32>) -> Option<i32> {
    // TODO: Implement this
    // Hint: iterate and return Some when you find it
    None
}

fn challenge_2() {
    // TODO: Implement safe division using Result
    // Test with: divide(10.0, 2.0) and divide(10.0, 0.0)

    // match safe_divide(10.0, 2.0) {
    //     Ok(result) => println!("Result: {}", result),
    //     Err(e) => println!("Error: {}", e),
    // }
}

fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    // TODO: Implement this
    // Return Err if b is 0.0, Ok(a/b) otherwise
    Ok(0.0)
}

fn challenge_3() {
    // TODO: Parse a vector of strings into integers
    // Some strings might not be valid numbers!
    // Use Result to handle parsing errors

    let data = vec!["42", "13", "invalid", "99"];

    // for s in data {
    //     match parse_number(s) {
    //         Ok(num) => println!("Parsed: {}", num),
    //         Err(e) => println!("Error parsing '{}': {}", s, e),
    //     }
    // }
}

fn parse_number(s: &str) -> Result<i32, String> {
    // TODO: Implement this
    // Hint: use s.parse::<i32>() which returns a Result
    // Convert the error to a String
    Ok(0)
}

// Advanced: Combining Option and Result
fn challenge_4() {
    // TODO: Find the mean of a vector
    // Return None if empty, Some(mean) otherwise

    let numbers = vec![10, 20, 30, 40, 50];
    let empty: Vec<i32> = vec![];

    // match calculate_mean(&numbers) {
    //     Some(mean) => println!("Mean: {}", mean),
    //     None => println!("Cannot calculate mean of empty vector"),
    // }
}

fn calculate_mean(numbers: &Vec<i32>) -> Option<f64> {
    // TODO: Implement this
    // Return None if empty, Some(sum / len) otherwise
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_greater() {
        assert_eq!(find_first_greater_than_50(&vec![10, 60, 20]), Some(60));
        assert_eq!(find_first_greater_than_50(&vec![10, 20, 30]), None);
    }

    #[test]
    fn test_safe_divide() {
        assert!(safe_divide(10.0, 2.0).is_ok());
        assert!(safe_divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse_number("42").unwrap(), 42);
        assert!(parse_number("abc").is_err());
    }

    #[test]
    fn test_mean() {
        assert_eq!(calculate_mean(&vec![10, 20, 30]), Some(20.0));
        assert_eq!(calculate_mean(&vec![]), None);
    }
}
