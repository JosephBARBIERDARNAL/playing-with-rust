// Exercise 01: Ownership
// Fix the ownership issues in these functions

pub fn exercise() {
    println!("\n=== Exercise 01: Ownership ===");

    // TODO: Fix this function - x is moved but we try to use it again
    let x = vec![1, 2, 3];
    let y = x;
    // println!("x: {:?}", x);  // Uncomment and fix!
    println!("y: {:?}", y);

    // TODO: Complete this function
    // Create two strings, pass one to takes_ownership, and print both
    // (Hint: you'll need to use .clone() for one of them)
    challenge_1();

    // TODO: Complete this function
    challenge_2();
}

fn takes_ownership(s: String) {
    println!("Function received: {}", s);
}

fn challenge_1() {
    // Create string s1 with value "Python"
    // Create string s2 with value "Rust"
    // Pass s1 to takes_ownership
    // Print both s1 and s2 after the function call
    // You'll need to think about cloning!
}

fn challenge_2() {
    // Create a vector of integers: vec![10, 20, 30, 40, 50]
    // Create a function that takes ownership and returns the sum
    // Call that function and print the result
    // Try to access the original vector after - what happens?
}

// Helper function for challenge_2
fn sum_vector(v: Vec<i32>) -> i32 {
    // TODO: Implement this
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership() {
        // This test will compile only when you fix the issues
        let v = vec![1, 2, 3];
        let v_clone = v.clone();
        takes_ownership(v.into_iter().map(|x| x.to_string()).collect());
        assert_eq!(v_clone, vec![1, 2, 3]);
    }
}
