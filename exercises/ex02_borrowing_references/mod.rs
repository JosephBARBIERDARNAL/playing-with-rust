// Exercise 02: Borrowing & References

pub fn exercise() {
    println!("\n=== Exercise 02: Borrowing & References ===");

    // TODO: Fix borrowing issues
    challenge_1();
    challenge_2();
    challenge_3();
}

fn challenge_1() {
    // TODO: Complete this function
    // Create a String called "data" with value "Data Science"
    // Pass it to get_length WITHOUT moving it
    // Print both the length and the original string

    let data = String::from("Data Science");
    // let length = get_length(???);  // How do you pass it?
    // println!("Length: {}, Data: {}", length, data);
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn challenge_2() {
    // TODO: Complete this function
    // Create a mutable vector: vec![1, 2, 3]
    // Pass it to add_values to add 4 and 5
    // Print the vector afterwards (should be [1, 2, 3, 4, 5])
}

fn add_values(v: &mut Vec<i32>) {
    v.push(4);
    v.push(5);
}

fn challenge_3() {
    // TODO: Fix the borrowing issue here
    // Uncomment and fix the code below

    // let mut scores = vec![85, 90, 78, 92];
    // let first_score = &scores[0];  // Immutable borrow
    // scores.push(88);  // ERROR: Can't mutate while borrowed!
    // println!("First score: {}", first_score);

    // Hint: Think about the scope of first_score
}

// Advanced challenge
fn challenge_4() {
    // TODO: Implement a function that takes a reference to a vector
    // and returns the average as f64
    // Don't take ownership of the vector!

    let data = vec![10, 20, 30, 40, 50];
    // let avg = calculate_average(???);
    // println!("Average: {}, Data still here: {:?}", avg, data);
}

fn calculate_average(v: &Vec<i32>) -> f64 {
    // TODO: Implement this
    // Hint: sum as f64 / length as f64
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrowing() {
        let s = String::from("test");
        assert_eq!(get_length(&s), 4);
        assert_eq!(s, "test");  // s still valid
    }

    #[test]
    fn test_average() {
        let data = vec![10, 20, 30];
        assert_eq!(calculate_average(&data), 20.0);
        assert_eq!(data.len(), 3);  // data still valid
    }
}
