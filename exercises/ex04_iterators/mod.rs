// Exercise 04: Iterators

pub fn exercise() {
    println!("\n=== Exercise 04: Iterators ===");

    challenge_1();
    challenge_2();
    challenge_3();
    challenge_4();
}

fn challenge_1() {
    // TODO: Use iterators to transform this data
    // Given a vector of temperatures in Celsius,
    // convert to Fahrenheit and collect into a new vector
    // Formula: F = C * 9/5 + 32

    let celsius = vec![0, 10, 20, 30, 40];
    // let fahrenheit: Vec<i32> = celsius
    //     .iter()
    //     .map(|&c| ???)
    //     .collect();
    // println!("Fahrenheit: {:?}", fahrenheit);
}

fn challenge_2() {
    // TODO: Filter and map in one chain
    // Given a vector of numbers, find all even numbers,
    // square them, and collect the results

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let result: Vec<i32> = ???
    // println!("Even squares: {:?}", result);
}

fn challenge_3() {
    // TODO: Calculate statistics using iterators
    // Given a vector of scores, calculate:
    // - Total sum
    // - Mean
    // - Count of scores above 80

    let scores = vec![75, 82, 90, 68, 95, 78, 88, 92];
    // let sum: i32 = ???
    // let mean: f64 = ???
    // let high_scores = ???
    // println!("Sum: {}, Mean: {:.2}, High scores: {}", sum, mean, high_scores);
}

fn challenge_4() {
    // TODO: Use find, any, and all
    let numbers = vec![2, 4, 6, 8, 10, 12];

    // Find first number greater than 7
    // let first_big = ???

    // Check if any number is odd
    // let has_odd = ???

    // Check if all numbers are positive
    // let all_positive = ???
}

// Advanced challenge: Implement your own data processing pipeline
fn challenge_5() {
    // TODO: Given a vector of (name, score) tuples,
    // filter students with score >= 70,
    // extract just their names,
    // and collect into a vector

    let students = vec![
        ("Alice", 85),
        ("Bob", 65),
        ("Carol", 92),
        ("Dave", 58),
        ("Eve", 78),
    ];

    // let passing: Vec<&str> = students
    //     .iter()
    //     .filter(???)
    //     .map(???)
    //     .collect();
    // println!("Passing students: {:?}", passing);
}

// Helper for processing data
fn process_data(data: &Vec<i32>) -> Vec<i32> {
    // TODO: Return a new vector with:
    // - Only values > 10
    // - Each multiplied by 2
    // - In descending order
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        let input = vec![5, 15, 8, 20, 12, 3];
        let result = process_data(&input);
        assert_eq!(result, vec![40, 30, 24]);
    }

    #[test]
    fn test_iterators() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter().sum();
        assert_eq!(sum, 15);

        let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }
}
