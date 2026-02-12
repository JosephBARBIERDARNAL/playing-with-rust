// Exercise 05: Closures

pub fn exercise() {
    println!("\n=== Exercise 05: Closures ===");

    challenge_1();
    challenge_2();
    challenge_3();
    challenge_4();
}

fn challenge_1() {
    // TODO: Create simple closures
    // Create a closure that:
    // 1. Takes a number and returns its square
    // 2. Takes two numbers and returns their sum

    let square = |x: i32| x * x;
    let add = |x: i32, y: i32| x + y;

    println!("Square of 4: {}", square(4));
    println!("Sum of 3 and 5: {}", add(3, 5));
}

fn challenge_2() {
    // TODO: Capture variables from environment
    // Create a closure that captures a multiplier from outside
    // and multiplies input by it

    let multiplier = 3;
    let multiply = |x: i32| x * multiplier;

    println!("{}", multiply(4)); // Should print 12
    println!("{}", multiply(5)); // Should print 15
}

fn challenge_3() {
    // TODO: Use closures with iterators
    // Given a vector of prices, apply a 20% discount to each

    let prices = vec![100.0, 50.0, 75.0, 120.0];
    let discount_rate = 0.20;

    let discounted: Vec<f64> = prices
        .iter()
        .map(|price| price - price * discount_rate) // Apply discount using closure
        .collect();

    println!("Discounted prices: {:?}", discounted);
}

fn challenge_4() {
    // TODO: Mutable capture
    // Create a counter closure that increments and returns a count

    let mut count = 0;
    let mut counter = || {
        count += 1;
        return count;
    };

    println!("{}", counter()); // 1
    println!("{}", counter()); // 2
    println!("{}", counter()); // 3
}

// Advanced: Using closures as function parameters
fn apply_to_vec<F>(data: &Vec<i32>, operation: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // TODO: Apply the operation to each element
    // and return a new vector
    data.iter().map(|&x| operation(x)).collect()
}

fn challenge_5() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Use apply_to_vec with different closures
    let doubled = apply_to_vec(&numbers, |x| x + 1);
    let squared = apply_to_vec(&numbers, |x| x * x);

    println!("Doubled: {:?}", doubled);
    println!("Squared: {:?}", squared);
}

// Advanced: Return a closure
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    // TODO: Return a closure that multiplies by factor
    // Hint: use 'move' keyword
    move |x| x * factor // Fix this
}

fn challenge_6() {
    let times_two = make_multiplier(2);
    let times_five = make_multiplier(5);

    println!("{}", times_two(10)); // 20
    println!("{}", times_five(10)); // 50
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_to_vec() {
        let data = vec![1, 2, 3];
        let result = apply_to_vec(&data, |x| x * 2);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_make_multiplier() {
        let times_three = make_multiplier(3);
        assert_eq!(times_three(4), 12);
    }
}
