// Exercise 09: Advanced Pattern Matching

pub fn exercise() {
    println!("\n=== Exercise 09: Advanced Pattern Matching ===");

    challenge_1();
    challenge_2();
    challenge_3();
    challenge_4();
}

fn challenge_1() {
    // TODO: Match on ranges and guards
    // Given a test score, categorize it:
    // 90-100: "A"
    // 80-89: "B"
    // 70-79: "C"
    // 60-69: "D"
    // below 60: "F"

    let score = 85;
    // let grade = match score {
    //     ???
    // };
    // println!("Score {}: Grade {}", score, grade);
}

fn challenge_2() {
    // TODO: Destructure tuples
    // Given points, determine their relationship

    let point1 = (0, 0);
    let point2 = (3, 4);

    // match (point1, point2) {
    //     ((0, 0), _) => println!("Point1 is origin"),
    //     (_, (0, 0)) => println!("Point2 is origin"),
    //     ((x1, y1), (x2, y2)) if x1 == x2 => println!("On same vertical line"),
    //     ((x1, y1), (x2, y2)) if y1 == y2 => println!("On same horizontal line"),
    //     _ => println!("Different positions"),
    // }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

fn challenge_3() {
    // TODO: Destructure structs
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        city: String::from("NYC"),
    };

    // match person {
    //     Person { name, age: a, .. } if a >= 18 => {
    //         println!("{} is an adult", name);
    //     }
    //     Person { name, .. } => {
    //         println!("{} is a minor", name);
    //     }
    // }
}

enum DataPoint {
    Empty,
    Single(f64),
    Pair(f64, f64),
    Series(Vec<f64>),
}

fn challenge_4() {
    // TODO: Match on custom enums with destructuring
    let data = DataPoint::Series(vec![1.0, 2.0, 3.0]);

    // match data {
    //     DataPoint::Empty => println!("No data"),
    //     DataPoint::Single(x) if x < 0.0 => println!("Negative: {}", x),
    //     DataPoint::Single(x) => println!("Value: {}", x),
    //     DataPoint::Pair(x, y) => println!("Pair: ({}, {})", x, y),
    //     DataPoint::Series(ref v) if v.len() > 5 => println!("Long series"),
    //     DataPoint::Series(v) => println!("Series of {} values", v.len()),
    // }
}

// Advanced: Use if let
fn challenge_5() {
    // TODO: Use if let to extract values
    let result: Result<i32, String> = Ok(42);

    // if let Ok(value) = result {
    //     println!("Success: {}", value);
    // }

    let numbers = vec![1, 2, 3];
    // if let Some(first) = numbers.first() {
    //     println!("First: {}", first);
    // }
}

// Advanced: while let for processing
fn challenge_6() {
    // TODO: Use while let to process a stack
    let mut stack = vec![1, 2, 3, 4, 5];

    // while let Some(value) = stack.pop() {
    //     println!("Popped: {}", value);
    //     if value == 3 {
    //         break;
    //     }
    // }
}

// Advanced: @ bindings
fn challenge_7() {
    // TODO: Use @ to bind and match simultaneously
    let age = 25;

    // match age {
    //     n @ 0..=12 => println!("Child: {}", n),
    //     n @ 13..=19 => println!("Teenager: {}", n),
    //     n @ 20..=64 => println!("Adult: {}", n),
    //     n => println!("Senior: {}", n),
    // }
}

// Advanced: Complex pattern in function parameter
fn process_result(result: &Result<Vec<i32>, String>) {
    // TODO: Match on the result and extract data
    // match result {
    //     Ok(v) if v.is_empty() => println!("Empty vector"),
    //     Ok(v) => println!("Got {} items", v.len()),
    //     Err(e) => println!("Error: {}", e),
    // }
}

fn challenge_8() {
    let good = Ok(vec![1, 2, 3]);
    let bad: Result<Vec<i32>, String> = Err("Failed".to_string());

    process_result(&good);
    process_result(&bad);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patterns() {
        // Add tests for your pattern matching
    }
}
